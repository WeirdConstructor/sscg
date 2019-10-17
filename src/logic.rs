use wlambda::VVal;
use std::rc::Rc;
use std::cell::RefCell;

pub type ObjectID = usize;

pub type EventCallback = Fn(&mut GameState, VVal);

#[derive(Clone)]
pub struct GameState {
    pub object_registry:    Rc<RefCell<ObjectRegistry>>,
    pub event_router:       Rc<RefCell<EventRouter>>,
}

impl GameState {
    pub fn get_ship(&mut self, id: ObjectID) -> Option<Rc<RefCell<Ship>>> {
        match self.object_registry.borrow_mut().get(id) {
            Some(Object::Ship(s)) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn get_system(&mut self, id: ObjectID) -> Option<Rc<RefCell<System>>> {
        match self.object_registry.borrow_mut().get(id) {
            Some(Object::System(s)) => Some(s.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
enum Object {
    None,
    Entity(Rc<RefCell<Entity>>),
    System(Rc<RefCell<System>>),
    Ship(Rc<RefCell<Ship>>),
}

#[derive(Debug, Clone)]
pub struct ObjectRegistry {
    objects:            std::vec::Vec<Object>,
    tick_time_ms:       f64,
}

impl ObjectRegistry {
    pub fn new() -> Self {
        ObjectRegistry {
            objects: std::vec::Vec::new(),
            tick_time_ms: 0.0,
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

    pub fn reg_cb<F>(&mut self, ev: String, f: F) where F: 'static + Fn(&mut GameState, VVal) {
        if let Some(cbs) = self.callbacks.get_mut(&ev) {
            cbs.push(Rc::new(f));
        } else {
            let mut cbs : std::vec::Vec<Rc<EventCallback>> = std::vec::Vec::new();
            cbs.push(Rc::new(f));
            self.callbacks.insert(ev, cbs);
        }
    }

    pub fn emit(&mut self, ev: String, args: VVal) {
        self.event_queue.push((ev, args));
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
    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8));
    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8));
    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32, color: (u8, u8, u8, u8));
    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32, fg: (u8, u8, u8, u8), bg: Option<(u8, u8, u8, u8)>, txt: &str);
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

#[derive(Debug, PartialEq, Copy, Clone)]
enum CargoType {
    Rocks,
}

#[derive(Debug, Clone)]
pub struct Cargo {
    typ:        CargoType,
    count:      i32,
}

// 10 ticks == 1 second
#[derive(Debug, Clone)]
pub struct Ship {
    pub id:             ObjectID,
    pub system:         ObjectID,
    pub name:           String,
    pub pos:            (i32, i32),
    pub speed_t:        i32, // 100:1 => speed_t * 10 is speed per second
    course_progress:    i32, // 100:1
    course:             Option<Course>,
    pub max_fuel:           i32, // 1:1
    pub fuel_t:             i32, // 100:1
    pub fuel_cost:          i32, // per tank fill
    pub fuel:               i32, // 100:1
    pub fuel_bill:          i32,
    pub capacity:           i32,
    pub cargo:              std::vec::Vec<Cargo>,
}

impl Ship {
    pub fn new(name: String, max_fuel: i32, capacity: i32) -> Self {
        Ship {
            name,
            max_fuel,
            capacity,
            cargo:         std::vec::Vec::new(),
            fuel:          max_fuel * 100,
            fuel_t: 10,
            fuel_cost:     100,
            fuel_bill:     0,
            course_progress: 0,
            speed_t:       200,
            course:        None,
            pos:           (0, 0),
            system:        0,
            id:            0,
        }
    }

    pub fn set_id(&mut self, id: ObjectID) { self.id = id; }

    pub fn set_system(&mut self, sys_id: ObjectID) { self.system = sys_id; }

    pub fn set_course_to(&mut self, x: i32, y: i32) {
        self.course = Some(Course::new(self.pos.0, self.pos.1, x, y));
        self.course_progress = 0;
    }

    pub fn tick(&mut self, er: &mut EventRouter) {
        if let Some(_) = self.course {
            self.fuel -= self.fuel_t;
            if self.fuel <= 0 {
                self.fuel = self.max_fuel * 100;
                self.fuel_bill += self.fuel_cost;
            }

            self.course_progress += self.speed_t;
            let d = self.course.unwrap().distance() * 100;
            if self.course_progress >= d {
                self.pos = self.course.unwrap().to;
                self.course = None;
                er.emit("ship_arrived".to_string(),
                    VVal::Int(self.id as i64));

            } else {
                self.pos = self.course.unwrap().interpolate(
                    self.course_progress as f64 / d as f64);
                er.emit("ship_travel".to_string(),
                    VVal::Int(self.id as i64));
            }

            println!("SHIP: pos={:?} dis={} cp={} fuel={}", self.pos, d, self.course_progress, self.fuel);
        }

        er.emit("ship_tick".to_string(),
            VVal::Int(self.id as i64));
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
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Entity {
    pub id:             ObjectID,
    pub typ:            SystemObject,
    pub x:              i32,
    pub y:              i32,
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
            is_highlighted: false
        }
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
    pub id: ObjectID,
    x:      i32,
    y:      i32,
    objects: std::vec::Vec<Rc<RefCell<Entity>>>,
}

impl System {
    pub fn new(x: i32, y: i32) -> Self {
        System { id: 0, x, y, objects: std::vec::Vec::new() }
    }

    pub fn set_id(&mut self, id: ObjectID) { self.id = id; }

    pub fn tick(&mut self, er: &mut EventRouter) {
        er.emit("system_tick".to_string(), VVal::Int(self.id as i64));
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

        if last_dist < 10_i32.pow(2) {
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
