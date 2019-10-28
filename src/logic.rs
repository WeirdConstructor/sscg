use wlambda::VVal;
use std::rc::Rc;
use std::cell::RefCell;

pub type ObjectID = usize;

pub type EventCallback = Fn(&Rc<RefCell<GameState>>, VVal);

#[derive(Clone)]
pub struct GameState {
    pub object_registry:    Rc<RefCell<ObjectRegistry>>,
    pub event_router:       Rc<RefCell<EventRouter>>,
    pub active_ship_id:     ObjectID,
    pub state:              VVal,
}

impl GameState {
    pub fn new_ref() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(GameState {
            object_registry: Rc::new(RefCell::new(ObjectRegistry::new())),
            event_router:    Rc::new(RefCell::new(EventRouter::new())),
            state:           VVal::map(),
            active_ship_id:  0,
        }))
    }

    pub fn serialize(&self) -> VVal {
        let objreg = self.object_registry.borrow().serialize();
        let v = VVal::vec();
        v.push(VVal::new_str("sscg_savegame"));
        v.push(VVal::Int(0)); // version
        v.push(self.state.clone());
        v.push(VVal::Int(self.active_ship_id as i64));
        v.push(objreg);
        return v;
    }

    pub fn deserialize(&mut self, v: VVal) {
        self.object_registry.borrow_mut().deserialize(v.at(4).unwrap_or(VVal::Nul));
        self.state          = v.at(2).unwrap_or(VVal::Nul);
        self.active_ship_id = v.at(3).unwrap_or(VVal::Nul).i() as ObjectID;
    }

    pub fn get_ship(&self, id: ObjectID) -> Option<Rc<RefCell<Ship>>> {
        match self.object_registry.borrow_mut().get(id) {
            Some(Object::Ship(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_system(&self, id: ObjectID) -> Option<Rc<RefCell<System>>> {
        match self.object_registry.borrow_mut().get(id) {
            Some(Object::System(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn add_system(&self, x: i32, y: i32, state: VVal) -> Rc<RefCell<System>> {
        for o in self.object_registry.borrow().objects.iter() {
            match o {
                Object::System(s) => {
                    return s.clone();
                },
                _ => (),
            }
        }

        let mut sys = System::new(x, y);
        sys.state = state;
        self.object_registry.borrow_mut().add_system(sys)
    }

    pub fn system_add_entity(
        &self, sys: Rc<RefCell<System>>,
        x: i32, y: i32, state: VVal) -> Rc<RefCell<Entity>> {

        let typ =
            match &state.get_key("type").unwrap_or(VVal::Nul).s_raw()[..] {
                "station"           => SystemObject::Station,
                "asteroid_field"    => SystemObject::AsteroidField,
                _                   => SystemObject::AsteroidField,
            };

        let mut ent = Entity::new(typ);
        ent.state = state;

        let e = self.object_registry.borrow_mut().add_entity(ent);
        sys.borrow_mut().add(x, y, e.clone());
        e
    }

    pub fn reg_cb<F>(&self, ev: String, f: F)
        where F: 'static + Fn(&Rc<RefCell<GameState>>, VVal) {
        self.event_router.borrow_mut().reg_cb(ev, f);
    }

    pub fn update(&self, frame_time_ms: f64) {
        let mut os = self.object_registry.borrow_mut();
        let mut er = self.event_router.borrow_mut();
        os.update(frame_time_ms, &mut *er);
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    None,
    Entity(Rc<RefCell<Entity>>),
    System(Rc<RefCell<System>>),
    Ship(Rc<RefCell<Ship>>),
}

impl Object {
    pub fn id(&self) -> ObjectID {
        match self {
            Object::None      => 0,
            Object::Entity(e) => e.borrow().id,
            Object::System(s) => s.borrow().id,
            Object::Ship(s)   => s.borrow().id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectRegistry {
    objects:            std::vec::Vec<Object>,
    tick_count:         i32,
    tick_time_ms:       f64,
}

impl ObjectRegistry {
    pub fn new() -> Self {
        ObjectRegistry {
            objects: std::vec::Vec::new(),
            tick_count:   0,
            tick_time_ms: 0.0,
        }
    }

    pub fn serialize(&self) -> VVal {
        let v = VVal::vec();
        v.push(VVal::Int(self.objects.len() as i64));

        let objs = VVal::vec();
        for o in self.objects.iter() {
            objs.push(match o {
                Object::Entity(e) => e.borrow().serialize(),
                Object::System(e) => e.borrow().serialize(),
                Object::Ship(e)   => e.borrow().serialize(),
                _                 => VVal::Nul,
            });
        }
        v.push(objs);

        v
    }

    fn vval_to_object(&mut self, v: VVal) -> Object {
        let typ : String = v.at(0).unwrap_or(VVal::Nul).s_raw();
        match &typ[..] {
            "ship"   => Object::Ship(Rc::new(RefCell::new(Ship::deserialize(self, v)))),
            "system" => Object::System(Rc::new(RefCell::new(System::deserialize(self, v)))),
            _ => Object::None,
        }
    }

    pub fn set_object_at(&mut self, idx: usize, o: Object) {
        println!("SET OBJ {} = {:?}", idx, o);
        self.objects[idx] = o;
    }

    pub fn deserialize(&mut self, s: VVal) {
        self.objects = std::vec::Vec::new();
        self.tick_time_ms = 0.0;

        self.objects.resize(
            s.at(0).unwrap_or(VVal::Int(0)).i() as usize,
            Object::None);

        if let VVal::Lst(m) = s.at(1).unwrap_or(VVal::Nul) {
            for v in m.borrow().iter() {
                let o = self.vval_to_object(v.clone());
                match o {
                    Object::None => (),
                    _ => self.set_object_at(o.id(), o),
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64, er: &mut EventRouter) {
        self.tick_time_ms += dt;
        //d// println!("UPD: {} {}", dt, self.tick_time_ms);
        while self.tick_time_ms > 100.0 {
            self.tick(er);
            self.tick_time_ms = self.tick_time_ms - 100.0;
        }
    }

    pub fn tick(&mut self, er: &mut EventRouter) {
        self.tick_count += 1;
        if self.tick_count > 5 {
            self.tick_count = 0;
            er.emit("tick".to_string(), VVal::Nul);
        }

        for o in self.objects.iter() {
            match o {
                Object::Ship(s)   => s.borrow_mut().tick(er),
                Object::System(s) => s.borrow_mut().tick(er),
                _ => (),
            }
        }
    }

    pub fn add_entity(&mut self, mut e: Entity) -> Rc<RefCell<Entity>> {
        e.set_id(self.objects.len());
        let r = Rc::new(RefCell::new(e));
        self.objects.push(Object::Entity(r.clone()));
        r
    }

    pub fn add_ship(&mut self, mut s: Ship) -> Rc<RefCell<Ship>> {
        s.set_id(self.objects.len());
        let r = Rc::new(RefCell::new(s));
        self.objects.push(Object::Ship(r.clone()));
        r
    }

    pub fn add_system(&mut self, mut s: System) -> Rc<RefCell<System>> {
        s.set_id(self.objects.len());
        let r = Rc::new(RefCell::new(s));
        self.objects.push(Object::System(r.clone()));
        r
    }

    pub fn get(&self, id: ObjectID) -> Option<Object> {
        if let Some(o) = self.objects.get(id) {
            match o {
                Object::None      => None,
                Object::Entity(_) => Some(o.clone()),
                Object::System(_) => Some(o.clone()),
                Object::Ship(_)   => Some(o.clone()),
            }
        } else {
            None
        }
    }
}

pub struct EventRouter {
    callbacks: std::collections::HashMap<String, std::vec::Vec<Rc<EventCallback>>>,
    event_queue: std::vec::Vec<(String, VVal)>,
}

impl EventRouter {
    pub fn new() -> Self {
        EventRouter {
            event_queue: std::vec::Vec::new(),
            callbacks:   std::collections::HashMap::new(),
        }
    }

    pub fn reg_cb<F>(&mut self, ev: String, f: F)
        where F: 'static + Fn(&Rc<RefCell<GameState>>, VVal) {

        if let Some(cbs) = self.callbacks.get_mut(&ev) {
            cbs.push(Rc::new(f));
        } else {
            let mut cbs : std::vec::Vec<Rc<EventCallback>> = std::vec::Vec::new();
            cbs.push(Rc::new(f));
            self.callbacks.insert(ev, cbs);
        }
    }

    pub fn emit(&mut self, ev: String, args: VVal) {
        if self.callbacks.get(&ev).is_none() {
            let a2 = VVal::vec();
            a2.push(VVal::new_str_mv(ev));
            a2.push(args);
            self.event_queue.push(("*".to_string(), a2));
        } else {
            self.event_queue.push((ev, args));
        }
    }

    pub fn get_events(&mut self, vec: &mut Vec<(Rc<EventCallback>, VVal)>) {
        while !self.event_queue.is_empty() {
            let ev = self.event_queue.pop().unwrap();
            if let Some(cbs) = self.callbacks.get_mut(&ev.0) {
                for c in cbs.iter() {
                    vec.push((c.clone(), ev.1.clone()));
                }
            }
        }
    }
}


pub trait GamePainter {
    fn push_offs(&mut self, xo: i32, yo: i32);
    fn push_add_offs(&mut self, xo: i32, yo: i32);
    fn pop_offs(&mut self);
    fn get_screen_pos(&self, xo: i32, yo: i32) -> (i32, i32);
    fn get_sprite_size(&self, id: usize) -> (u32, u32);
    fn draw_sprite(&mut self, xo: i32, yo: i32, w: u32, h: u32, id: usize) {
        self.draw_sprite_ex(xo, yo, w, h, id,  0.0, false, false);
    }
    fn draw_sprite_ex(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                      id: usize, angle: f64, flip_h: bool, flip_v: bool);
    fn disable_clip_rect(&mut self);
    fn set_clip_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32);
    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                 color: (u8, u8, u8, u8));
    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                        color: (u8, u8, u8, u8));
    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32,
                 color: (u8, u8, u8, u8));
    fn text_size(&mut self, txt: &str) -> (u32, u32);
    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32,
                 fg: (u8, u8, u8, u8),
                 bg: Option<(u8, u8, u8, u8)>,
                 align: i32,
                 txt: &str);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SystemObject {
    Station,
    AsteroidField,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Course {
    pub from: (i32, i32),
    pub to:   (i32, i32),
}

impl Course {
    pub fn new(x_from: i32, y_from: i32, x_to: i32, y_to: i32) -> Self {
        Course {
            from: (x_from, y_from),
            to: (x_to, y_to),
        }
    }

    pub fn interpolate(&self, v: f64) -> (i32, i32) {
        let xd = ((self.to.0 as f64 * v) + (self.from.0 as f64 * (1.0 - v))) as i32;
        let yd = ((self.to.1 as f64 * v) + (self.from.1 as f64 * (1.0 - v))) as i32;
        (xd, yd)
    }

    pub fn distance(&self) -> i32 {
        ((  (self.from.0 - self.to.0).pow(2)
         + (self.from.1 - self.to.1).pow(2)) as f64).sqrt() as i32
    }
}

// 10 ticks == 1 second
#[derive(Debug, Clone)]
pub struct Ship {
    pub id:             ObjectID,
    pub system:         ObjectID,
    pub name:           String,
    pub notify_txt:     String,
    pub pos:            (i32, i32),
    pub speed_t:        i32, // 100:1 => speed_t * 10 is speed per second
    pub state:          VVal,
    course_progress:    i32, // 100:1
    course:             Option<Course>,
    tick_count:         i32,
}

impl Ship {
    pub fn new(name: String) -> Self {
        Ship {
            name,
            course_progress:    0,
            speed_t:            200,
            course:             None,
            pos:                (0, 0),
            system:             0,
            id:                 0,
            state:              VVal::map(),
            tick_count:         0,
            notify_txt:         String::from(""),
        }
    }

    pub fn deserialize(_or: &mut ObjectRegistry, v: VVal) -> Self {
        let mut s = Self::new("".to_string());
        s.id                = v.at(2).unwrap_or(VVal::Int(0)).i() as ObjectID;
        s.system            = v.at(3).unwrap_or(VVal::Int(0)).i() as ObjectID;
        s.name              = v.at(4).unwrap_or(VVal::new_str("")).s_raw();
        s.pos.0             = v.at(5).unwrap_or(VVal::Int(0)).i() as i32;
        s.pos.1             = v.at(6).unwrap_or(VVal::Int(0)).i() as i32;
        s.speed_t           = v.at(7).unwrap_or(VVal::Int(0)).i() as i32;
        s.course_progress   = v.at(8).unwrap_or(VVal::Int(0)).i() as i32;
        if let Some(VVal::Lst(l)) = v.at(9) {
            let mut c = Course::new(0, 0, 0, 0);
            c.from.0 = l.borrow().get(0).unwrap().i() as i32;
            c.from.1 = l.borrow().get(1).unwrap().i() as i32;
            c.to.0   = l.borrow().get(2).unwrap().i() as i32;
            c.to.1   = l.borrow().get(3).unwrap().i() as i32;
            s.course = Some(c);
        } else {
            s.course = None;
        }
        s.tick_count        = v.at(10).unwrap_or(VVal::Int(0)).i() as i32;
        s.state             = v.at(11).unwrap_or(VVal::Nul);
        s
    }

    pub fn serialize(&self) -> VVal {
        let v = VVal::vec();
        v.push(VVal::new_str("ship"));
        v.push(VVal::Int(0)); // version
        v.push(VVal::Int(self.id      as i64));
        v.push(VVal::Int(self.system  as i64));
        v.push(VVal::new_str(&self.name));
        v.push(VVal::Int(self.pos.0   as i64));
        v.push(VVal::Int(self.pos.1   as i64));
        v.push(VVal::Int(self.speed_t as i64));
        v.push(VVal::Int(self.course_progress as i64));
        if let Some(c) = self.course {
            let cv = VVal::vec();
            cv.push(VVal::Int(c.from.0 as i64));
            cv.push(VVal::Int(c.from.1 as i64));
            cv.push(VVal::Int(c.to.0 as i64));
            cv.push(VVal::Int(c.to.1 as i64));
            v.push(cv);
        } else {
            v.push(VVal::Nul);
        }
        v.push(VVal::Int(self.tick_count  as i64));
        v.push(self.state.clone());
        v
    }

    pub fn set_id(&mut self, id: ObjectID) { self.id = id; }

    pub fn set_system(&mut self, sys_id: ObjectID) { self.system = sys_id; }

    pub fn set_notification(&mut self, not: String) { self.notify_txt = not; }

    pub fn set_course_to(&mut self, x: i32, y: i32) {
        self.course = Some(Course::new(self.pos.0, self.pos.1, x, y));
        self.course_progress = 0;
    }

    pub fn tick(&mut self, er: &mut EventRouter) {
        if let Some(_) = self.course {
            self.course_progress += self.speed_t;
            let d = self.course.unwrap().distance() * 100;
            if self.course_progress >= d {
                self.pos = self.course.unwrap().to;
                self.course = None;
                self.state.set_map_key(
                    "_state".to_string(), VVal::new_str("arrived"));

            } else {
                self.pos = self.course.unwrap().interpolate(
                    self.course_progress as f64 / d as f64);
                self.state.set_map_key(
                    "_state".to_string(), VVal::new_str("flying"));
            }

            println!("SHIP: pos={:?} dis={} cp={}", self.pos, d, self.course_progress);
        } else {
            self.state.set_map_key(
                "_state".to_string(), VVal::new_str("stopped"));
        }

        self.tick_count += 1;
        if self.tick_count > 5 {
            self.tick_count = 0;
            er.emit("ship_tick".to_string(),
                VVal::Int(self.id as i64));
        }
    }

    pub fn draw<P>(&mut self, p: &mut P) where P: GamePainter {
        if let Some(c) = self.course {
            p.draw_line(
                self.pos.0,
                self.pos.1,
                c.to.0,
                c.to.1,
                1,
                (190, 190, 190, 255));
        }

        p.draw_dot(
            self.pos.0,
            self.pos.1,
            3,
            (160, 160, 255, 255));

        if self.notify_txt.len() > 0 {
            p.draw_text(
                self.pos.0 - 100, self.pos.1 + 10, 200,
                (255, 0, 255, 255), None,
                0, &self.notify_txt);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id:             ObjectID,
    pub typ:            SystemObject,
    pub x:              i32,
    pub y:              i32,
    pub state:          VVal,
    draw_pos:           (i32, i32),
    is_highlighted:     bool,
}

impl Entity {
    pub fn new(typ: SystemObject) -> Self {
        Entity {
            id: 0,
            typ,
            draw_pos: (0, 0),
            x: 0,
            y: 0,
            state: VVal::map(),
            is_highlighted: false
        }
    }

    pub fn deserialize(_or: &mut ObjectRegistry, v: VVal) -> Self {
        let mut s = Self::new(SystemObject::Station);
        s.id            = v.at(2).unwrap_or(VVal::Int(0)).i() as ObjectID;
        s.typ           = match v.at(3).unwrap_or(VVal::Int(0)).i() {
                              0 => SystemObject::Station,
                              1 => SystemObject::AsteroidField,
                              _ => SystemObject::Station,
                          };
        s.x             = v.at(4).unwrap_or(VVal::Int(0)).i() as i32;
        s.y             = v.at(5).unwrap_or(VVal::Int(0)).i() as i32;
        s.state         = v.at(6).unwrap_or(VVal::Nul);
        s
    }

    pub fn serialize(&self) -> VVal {
        let v = VVal::vec();
        v.push(VVal::new_str("entity"));
        v.push(VVal::Int(0)); // version
        v.push(VVal::Int(self.id  as i64));
        v.push(VVal::Int(self.typ as i64));
        v.push(VVal::Int(self.x   as i64));
        v.push(VVal::Int(self.y   as i64));
        v.push(self.state.clone());
        v
    }

    pub fn set_id(&mut self, id: ObjectID) { self.id = id; }

    fn draw<P>(&mut self, p: &mut P) where P: GamePainter {
        if self.is_highlighted {
            p.draw_circle(0, 0, 8, (255, 0, 0, 255));
        }
        match self.typ {
            SystemObject::Station => {
                p.draw_dot(0, 0, 4, (0, 190, 0, 255));
            },
            SystemObject::AsteroidField => {
                p.draw_dot(0, 0, 2, (190, 190, 190, 255));
            },
        }
        self.draw_pos = p.get_screen_pos(0, 0);
    }

    fn set_highlight(&mut self, h: bool) {
        self.is_highlighted = h;
    }
}

#[derive(Debug, Clone)]
pub struct System {
    pub id:     ObjectID,
    pub x:      i32,
    pub y:      i32,
    pub state:  VVal,
    objects:    std::vec::Vec<Rc<RefCell<Entity>>>,
    tick_count: i32,
}

impl System {
    pub fn new(x: i32, y: i32) -> Self {
        System {
            id: 0,
            x,
            y,
            objects: std::vec::Vec::new(),
            tick_count: 0,
            state: VVal::map()
        }
    }

    pub fn deserialize(or: &mut ObjectRegistry, v: VVal) -> Self {
        let mut s = Self::new(0, 0);
        s.id            = v.at(2).unwrap_or(VVal::Int(0)).i() as ObjectID;
        s.x             = v.at(3).unwrap_or(VVal::Int(0)).i() as i32;
        s.y             = v.at(4).unwrap_or(VVal::Int(0)).i() as i32;
        s.tick_count    = v.at(5).unwrap_or(VVal::Int(0)).i() as i32;
        s.state         = v.at(6).unwrap_or(VVal::Nul);
        if let Some(VVal::Lst(l)) = v.at(7) {
            for o in l.borrow().iter() {
                let e = Rc::new(RefCell::new(Entity::deserialize(or, o.clone())));
                let id = e.borrow().id;
                or.set_object_at(id, Object::Entity(e.clone()));
                s.objects.push(e);
            }
        }
        s
    }

    pub fn serialize(&self) -> VVal {
        let v = VVal::vec();
        v.push(VVal::new_str("system"));
        v.push(VVal::Int(0)); // version
        v.push(VVal::Int(self.id          as i64));
        v.push(VVal::Int(self.x           as i64));
        v.push(VVal::Int(self.y           as i64));
        v.push(VVal::Int(self.tick_count  as i64));
        v.push(self.state.clone());
        let o = VVal::vec();
        for obj in self.objects.iter() {
            o.push(obj.borrow().serialize());
        }
        v.push(o);
        v
    }

    pub fn set_id(&mut self, id: ObjectID) { self.id = id; }

    pub fn tick(&mut self, er: &mut EventRouter) {
        self.tick_count += 1;
        if self.tick_count > 10 {
            self.tick_count = 0;
            er.emit("system_tick".to_string(), VVal::Int(self.id as i64));
        }
    }

    pub fn add(&mut self, x: i32, y: i32, e: Rc<RefCell<Entity>>) {
        e.borrow_mut().x = x;
        e.borrow_mut().y = y;
        self.objects.push(e);
    }

    pub fn clear_entity_highlights(&mut self) {
        for e in self.objects.iter_mut() {
            e.borrow_mut().set_highlight(false);
        }
    }

    pub fn try_highlight_entity_close_to(&mut self, x_screen: i32, y_screen: i32) {
        self.clear_entity_highlights();
        if let Some(e) = self.get_entity_close_to_screen(x_screen, y_screen) {
            e.borrow_mut().set_highlight(true);
        }
    }

    pub fn get_entity_close_to(&mut self, x: i32, y: i32) -> Option<Rc<RefCell<Entity>>> {
        let mut closest_i : i32 = -1;
        let mut last_dist : i32 = 99999;

        for (i, ent) in self.objects.iter().enumerate() {
            let ent_r = ent.borrow_mut();
            let d : i32 = (ent_r.x - x).pow(2)
                        + (ent_r.y - y).pow(2);
            if d < last_dist {
                last_dist = d;
                closest_i = i as i32;
            }
        }

        if last_dist < 2_i32.pow(2) {
            return self.objects.get(closest_i as usize).cloned();
        }
        return None;
    }

    pub fn get_entity_close_to_screen(&mut self, x_screen: i32, y_screen: i32) -> Option<Rc<RefCell<Entity>>> {
        let mut closest_i : i32 = -1;
        let mut last_dist : i32 = 99999;

        for (i, ent) in self.objects.iter().enumerate() {
            let ent_r = ent.borrow_mut();
            let d : i32 = (ent_r.draw_pos.0 - x_screen).pow(2)
                        + (ent_r.draw_pos.1 - y_screen).pow(2);
            if d < last_dist {
                last_dist = d;
                closest_i = i as i32;
            }
        }

        if last_dist < 20_i32.pow(2) {
            return self.objects.get(closest_i as usize).cloned();
        }
        return None;
    }

    pub fn draw<P>(&mut self, ship: &mut Ship, p: &mut P) where P: GamePainter {
        let cell_count = 10;
        let cell_size  = 48;
        let w = cell_count * cell_size;
        p.draw_rect_filled(
            0, 0,
            cell_size * cell_count, cell_size * cell_count,
            (0, 0, 50, 255));
        for h_line_y in 0..11 {
            p.draw_line(0, h_line_y * 48, (w - 1) as i32, h_line_y * 48, 1, (200, 0, 0, 255));
        }
        for v_line_x in 0..11 {
            p.draw_line(v_line_x * 48, 0, v_line_x * 48, (w - 1) as i32, 1, (200, 0, 0, 255));
        }

        for ent in self.objects.iter_mut() {
            p.push_add_offs(ent.borrow().x, ent.borrow().y);
            ent.borrow_mut().draw(p);
            p.pop_offs();
        }

        if ship.system == self.id {
            ship.draw(p);
        }
    }
}
