use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};
//use vecmath::*;

mod logic;
mod util;
mod gui;
mod sdl_painter;
use logic::*;
use sdl_painter::SDLPainter;

use wlambda::{VVal, StackAction, VValUserData, GlobalEnv, EvalContext};

fn vval_to_system(v: VVal) -> Result<Rc<RefCell<System>>, StackAction> {
    match v {
        VVal::Usr(mut ud) => {
            if let Some(sys) = ud.as_any().downcast_ref::<SystemWlWrapper>() {
                Ok(sys.0.clone())
            } else {
                Err(StackAction::panic_msg(
                    format!("{} is not a system", ud.s())))
            }
        },
        _ => {
            Err(StackAction::panic_msg(
                format!("{} is not a system", v.s())))
        }
    }
}

pub fn get_system_state(gs: &GameState, id: ObjectID) -> Option<VVal> {
    let system : Rc<RefCell<System>> = gs.get_system(id)?;

    let ret = VVal::vec();
    ret.push(SystemWlWrapper::vval_from(system));

    Some(ret)
}

pub fn get_ship_state(gs: &GameState, id: ObjectID) -> Option<VVal> {
    let ship   : Rc<RefCell<Ship>>   = gs.get_ship(id)?;
    let system : Rc<RefCell<System>> = gs.get_system(ship.borrow().system)?;

    let entity =
        system.borrow_mut().get_entity_close_to(
            ship.borrow().pos.0,
            ship.borrow().pos.1);

    let ret = VVal::vec();
    ret.push(ShipWlWrapper::vval_from(ship));
    ret.push(SystemWlWrapper::vval_from(system));

    if let Some(ent) = entity {
        ret.push(EntityWlWrapper::vval_from(ent));
    }

    Some(ret)
}


#[derive(Clone)]
struct GameStateWlWrapper(Rc<RefCell<GameState>>);
impl GameStateWlWrapper {
    pub fn vval_from(r: Rc<RefCell<GameState>>) -> VVal {
        VVal::Usr(Box::new(GameStateWlWrapper(r)))
    }
}

impl VValUserData for GameStateWlWrapper {
    fn s(&self) -> String { format!("$<GameState>") }
    fn set_key(&self, _key: &VVal, _val: VVal) {
//        self.0.borrow_mut().state.set_key(key, val);
    }
    fn get_key(&self, key: &str) -> Option<VVal> {
        match key {
            _    => None,
        }
    }
    fn call(&self, args: &[VVal]) -> Result<VVal, StackAction> {
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            "add_entity" => {
                if args.len() < 5 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :add_entity` called with too few arguments",
                                self.s())));
                }
                let sys = vval_to_system(args[1].clone())?;
                Ok(EntityWlWrapper::vval_from(
                    self.0.borrow_mut().system_add_entity(
                        sys,
                        args[2].i() as i32,
                        args[3].i() as i32,
                        args[4].clone())))
            },
            "add_system" => {
                if args.len() < 4 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :add_system` called with too few arguments",
                                self.s())));
                }
                Ok(SystemWlWrapper::vval_from(
                    self.0.borrow_mut().add_system(
                        args[1].i() as i32,
                        args[2].i() as i32,
                        args[3].clone())))
            },
            "object_by_id" => {
                if args.len() < 2 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :object_by_id` called with too few arguments",
                                self.s())));
                }
                let o =
                    self.0.borrow_mut()
                        .object_registry.borrow_mut()
                        .get(args[1].i() as ObjectID);
                if let Some(obj) = o {
                    Ok(match obj {
                        Object::Entity(e) => EntityWlWrapper::vval_from(e),
                        Object::System(s) => SystemWlWrapper::vval_from(s),
                        Object::Ship(s)   => ShipWlWrapper::vval_from(s),
                        Object::None      => VVal::Nul,
                    })
                } else {
                    Ok(VVal::Nul)
                }
            },
            _ => Ok(VVal::Nul)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct EntityWlWrapper(Rc<RefCell<Entity>>);
impl EntityWlWrapper {
    pub fn vval_from(r: Rc<RefCell<Entity>>) -> VVal {
        VVal::Usr(Box::new(EntityWlWrapper(r)))
    }
}

impl VValUserData for EntityWlWrapper {
    fn s(&self) -> String { format!("$<Entity:{}>", self.0.borrow().id) }
    fn i(&self) -> i64 { self.0.borrow().id as i64 }
    fn set_key(&self, key: &VVal, val: VVal) {
        self.0.borrow_mut().state.set_key(key, val);
    }
    fn get_key(&self, key: &str) -> Option<VVal> {
        match key {
            "id" => Some(VVal::Int(self.0.borrow().id as i64)),
            "typ" => {
                Some(VVal::new_str(
                    match self.0.borrow().typ {
                        SystemObject::Station       => "station",
                        SystemObject::AsteroidField => "asteroid_field",
                    }))
            },
            _    => self.0.borrow().state.get_key(key),
        }
    }
    fn call(&self, args: &[VVal]) -> Result<VVal, StackAction> {
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            _            => Ok(VVal::Nul)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct ShipWlWrapper(Rc<RefCell<Ship>>);
impl ShipWlWrapper {
    pub fn vval_from(r: Rc<RefCell<Ship>>) -> VVal {
        VVal::Usr(Box::new(ShipWlWrapper(r)))
    }
}

impl VValUserData for ShipWlWrapper {
    fn s(&self) -> String { format!("$<Ship:{}>", self.0.borrow().id) }
    fn i(&self) -> i64 { self.0.borrow().id as i64 }
    fn set_key(&self, key: &VVal, val: VVal) {
        self.0.borrow_mut().state.set_key(key, val);
    }
    fn get_key(&self, key: &str) -> Option<VVal> {
        println!("GET KEY: {} : STTE: {}", key, self.0.borrow().state.s());
        match key {
            "id"        => Some(VVal::Int(self.0.borrow().id as i64)),
            "system_id" => Some(VVal::Int(self.0.borrow().system as i64)),
            _ => self.0.borrow().state.get_key(key),
        }
    }
    fn call(&self, args: &[VVal]) -> Result<VVal, StackAction> {
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            "set_notification" => {
                if args.len() < 2 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_system` called with too few arguments",
                                self.s())));
                }

                let txt = args[1].s_raw();

                self.0.borrow_mut().set_notification(txt);
                Ok(VVal::Bol(true))
            },
            "set_system" => {
                if args.len() < 2 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_system` called with too few arguments",
                                self.s())));
                }

                let sys = vval_to_system(args[1].clone())?;

                self.0.borrow_mut().system = sys.borrow().id;
                Ok(VVal::Bol(true))
            },
            _ => Ok(VVal::Nul)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct SystemWlWrapper(Rc<RefCell<System>>);
impl SystemWlWrapper {
    pub fn vval_from(r: Rc<RefCell<System>>) -> VVal {
        VVal::Usr(Box::new(SystemWlWrapper(r)))
    }
}

impl VValUserData for SystemWlWrapper {
    fn s(&self) -> String { format!("$<System:{}>", self.0.borrow().id) }
    fn i(&self) -> i64 { self.0.borrow().id as i64 }
    fn set_key(&self, key: &VVal, val: VVal) {
        self.0.borrow_mut().state.set_key(key, val);
    }
    fn get_key(&self, key: &str) -> Option<VVal> {
        match key {
            "id"    => Some(VVal::Int(self.0.borrow().id as i64)),
            _       => self.0.borrow().state.get_key(key),
        }
    }
    fn call(&self, args: &[VVal]) -> Result<VVal, StackAction> {
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            "foo" => { Ok(VVal::Bol(true)) },
            _     => Ok(VVal::Nul)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

struct WindowManager {
    windows: std::vec::Vec<Option<gui::Window>>,
    ev_cbs: std::vec::Vec<VVal>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: std::vec::Vec::new(),
            ev_cbs: std::vec::Vec::new(),
        }
    }

    pub fn handle_activated_childs(&mut self, wl_ctx: &mut EvalContext) {
        for (w, cb) in self.windows.iter_mut().zip(self.ev_cbs.iter()) {
            if let Some(w) = w {
                if let Some(lblref) = w.collect_activated_child() {
                    let args = vec![VVal::new_str_mv(lblref.to_string())];
                    if let Err(e) = wl_ctx.clone().call(cb, &args) {
                        println!("ERROR IN WM CB: {}", e);
                    }
                }
            }
        }
    }

    pub fn set_label_text(&mut self, idx: usize, lblref: &str, text: String) {
        if idx >= self.windows.len() {
            return;
        }
        if self.windows[idx].is_none() {
            return;
        }

        if let Some(e) = self.windows.get_mut(idx) {
            e.as_mut().unwrap().set_label_text(lblref, text);
        }
    }

    pub fn get_window_state(&self, idx: usize) -> VVal {
        if idx >= self.windows.len() {
            return VVal::Nul;
        }
        if self.windows[idx].is_none() {
            return VVal::Nul;
        }

        if let Some(e) = self.windows.get(idx) {
            let txts = e.as_ref().unwrap().collect_label_texts();
            let s = VVal::map();
            let v = VVal::map();
            for (lblref, text) in txts.into_iter() {
                v.set_map_key(lblref, VVal::new_str_mv(text));
            }
            s.set_map_key("labels".to_string(), v);
            s
        } else {
            VVal::Nul
        }
    }

    pub fn get_label_text(&self, idx: usize, lblref: &str) -> VVal {
        if idx >= self.windows.len() {
            return VVal::Nul;
        }
        if self.windows[idx].is_none() {
            return VVal::Nul;
        }

        if let Some(e) = self.windows.get(idx) {
            let r = e.as_ref().unwrap().get_label_text(lblref);
            if r.is_some() {
                VVal::new_str_mv(r.unwrap())
            } else {
                VVal::Nul
            }
        } else {
            VVal::Nul
        }
    }

    pub fn new_ref() -> std::rc::Rc<std::cell::RefCell<Self>> {
        std::rc::Rc::new(std::cell::RefCell::new(Self::new()))
    }

    pub fn delete(&mut self, idx: usize) {
        if idx >= self.windows.len() { return; }
        self.windows[idx] = None;
        self.ev_cbs[idx] = VVal::Nul;
    }

    pub fn set(&mut self, idx: usize, win: gui::Window, cb: VVal) -> usize {
        if idx >= self.windows.len() {
            self.windows.resize(idx + 1, None);
            self.ev_cbs.resize(idx + 1, VVal::Nul);
        }
        println!("SET WINDOW {}", idx);
        self.windows[idx] = Some(win);
        self.ev_cbs[idx]  = cb;
        idx
    }
}

#[derive(Clone)]
struct WindowManagerWlWrapper(Rc<RefCell<WindowManager>>);
impl WindowManagerWlWrapper {
    pub fn vval_from(r: Rc<RefCell<WindowManager>>) -> VVal {
        VVal::Usr(Box::new(WindowManagerWlWrapper(r)))
    }
}

fn shiftaddup4(u: u8) -> u8 { (u << 4) | u }

fn color_hex24tpl(s: &str) -> (u8, u8, u8, u8) {
    match s.len() {
        8 => (
            u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
            u8::from_str_radix(&s[6..8], 16).unwrap_or(255)
        ),
        6 => (
            u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
            255
        ),
        4 => (
            shiftaddup4(u8::from_str_radix(&s[0..1], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[1..2], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[2..3], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[3..4], 16).unwrap_or(0xF)),
        ),
        3 => (
            shiftaddup4(u8::from_str_radix(&s[0..1], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[1..2], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[2..3], 16).unwrap_or(0)),
            255
        ),
        _ => (255, 0, 255, 255),
    }
}

fn vval2size(v: VVal) -> gui::Size {
    let mut s = gui::Size {
        min_w: 0,
        min_h: 0,
        w:     1000,
        h:     1000,
        margin: 0,
    };

    s.min_w  = v.get_key("min_w") .unwrap_or(VVal::Int(0)).i() as u32;
    s.min_h  = v.get_key("min_h") .unwrap_or(VVal::Int(0)).i() as u32;
    s.w      = v.get_key("w")     .unwrap_or(VVal::Int(1000)).i() as u32;
    s.h      = v.get_key("h")     .unwrap_or(VVal::Int(1000)).i() as u32;
    s.margin = v.get_key("margin").unwrap_or(VVal::Int(0)).i() as u32;

    s
}

fn vval2widget(v: VVal, win: &mut gui::Window) -> usize {
    let mut childs = vec![];
    if let Some(VVal::Lst(l)) = v.get_key("childs") {
        for w in l.borrow().iter() {
            childs.push(vval2widget(w.clone(), win));
        }
    }

    match &v.get_key("t").unwrap_or(VVal::Nul).s_raw()[..] {
        "vbox" => {
            return win.add_layout(
                vval2size(v.clone()),
                gui::BoxDir::Vert(
                    v.get_key("spacing").unwrap_or(VVal::Int(0)).i() as u32),
                &childs);
        },
        "hbox" => {
            return win.add_layout(
                vval2size(v.clone()),
                gui::BoxDir::Hori(
                    v.get_key("spacing").unwrap_or(VVal::Int(0)).i() as u32),
                &childs);
        },
        _ => ()
    }

    let lbl =
        gui::Label::new(
            &v.get_key("text").unwrap_or(VVal::new_str("")).s_raw(),
            color_hex24tpl(
                &v.get_key("fg").unwrap_or(VVal::new_str("")).s_raw()),
            color_hex24tpl(
                &v.get_key("bg").unwrap_or(VVal::new_str("")).s_raw()))
        .lblref(&v.get_key("ref").unwrap_or(VVal::new_str("")).s_raw());

    let lbl = match &v.get_key("t").unwrap_or(VVal::Nul).s_raw()[..] {
        "l_button" => lbl.left().clickable(),
        "r_button" => lbl.right().clickable(),
        "c_button" => lbl.center().clickable(),
        "field" => {
            lbl.left().editable(
                &v.get_key("regex").unwrap_or(VVal::new_str(".*")).s_raw())
        },
        "c_label" => lbl.center(),
        "l_label" => lbl.left(),
        "r_label" => lbl.right(),
        _ => lbl,
    };

    win.add_label(vval2size(v.clone()), lbl)
}

fn vval2win(v: VVal) -> gui::Window {
    let mut w = gui::Window::new();
    w.x     = v.get_key("x").unwrap_or(VVal::Int(0)).i() as i32;
    w.y     = v.get_key("y").unwrap_or(VVal::Int(0)).i() as i32;
    w.w     = v.get_key("w").unwrap_or(VVal::Int(500)).i() as i32;
    w.h     = v.get_key("h").unwrap_or(VVal::Int(500)).i() as i32;
    w.title = v.get_key("title").unwrap_or(VVal::new_str("Unnamed")).s_raw();
    if let Some(tc) = v.get_key("title_color") {
        w.title_color = color_hex24tpl(&tc.s_raw());
    }
    let id = vval2widget(v.get_key("child").unwrap_or(VVal::Nul), &mut w);
    w.child = id;

    w
}

impl VValUserData for WindowManagerWlWrapper {
    fn s(&self) -> String { format!("$<WindowManager>") }
    fn i(&self) -> i64 { 0 }
    fn call(&self, args: &[VVal]) -> Result<VVal, StackAction> {
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            "set_window" => {
                if args.len() < 4 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_window` called with too few arguments",
                                self.s())));
                }


                let idx = args[1].i();
                if !args[2].is_none() {
                    let win = vval2win(args[2].clone());
                    let cb  = args[3].clone();

                    self.0.borrow_mut().set(idx as usize, win, cb);
                } else {
                    self.0.borrow_mut().delete(idx as usize);
                }

                Ok(VVal::Bol(true))
            },
            "set_label" => {
                if args.len() < 3 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_label` called with too few arguments",
                                self.s())));
                }

                let idx    = args[1].i();
                let lblref = args[2].s_raw();
                let txt    = args[3].s_raw();
                self.0.borrow_mut().set_label_text(idx as usize, &lblref, txt);

                Ok(VVal::Bol(true))
            },
            "get_label" => {
                if args.len() < 2 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :get_label` called with too few arguments",
                                self.s())));
                }

                let idx    = args[1].i();
                let lblref = args[2].s_raw();
                Ok(self.0.borrow_mut().get_label_text(idx as usize, &lblref))
            },
            "get_state" => {
                if args.len() < 1 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :get_label` called with too few arguments",
                                self.s())));
                }

                let idx = args[1].i();
                Ok(self.0.borrow_mut().get_window_state(idx as usize))
            },
            _ => Ok(VVal::Nul)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .resizable()
//        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let mut font = ttf_ctx.load_font("DejaVuSansMono.ttf", 14).map_err(|e| e.to_string())?;
//    font.set_style(sdl2::ttf::FontStyle::BOLD | sdl2::ttf::FontStyle::UNDERLINE);
    font.set_hinting(sdl2::ttf::Hinting::Normal);
//    font.set_outline_width(0.1);
    font.set_kerning(true);

    let mut sdl_painter = SDLPainter {
        canvas: canvas,
        font: Rc::new(RefCell::new(font)),
        font_h: 0,
        offs_stack: std::vec::Vec::new(),
        offs: (0, 0),
    };

    let s_wm = WindowManager::new_ref();
    let s_gs = GameState::new_ref();

    let genv = GlobalEnv::new_default();
    let mut wl_ctx = EvalContext::new_with_user(genv, s_gs.clone());

    wl_ctx.set_global_var("game", &GameStateWlWrapper::vval_from(s_gs.clone()));
    wl_ctx.set_global_var("win", &WindowManagerWlWrapper::vval_from(s_wm.clone()));

    let callbacks : VVal =
        match wl_ctx.eval_file("main.wl") {
            Ok(v) => {
                if v.is_err() {
                    panic!(format!("'main.wl' SCRIPT ERROR: {}", v.s()));
                } else {
                    println!("GET VALUE: {}", v.to_json(false).unwrap());
                    v.clone()
                }
            },
            Err(e) => { panic!(format!("'main.wl' SCRIPT ERROR: {}", e)); }
        };

    let wlcb_ship_tick =
        callbacks.get_key("ship_tick")
                 .expect("ship_tick key");
    let wlcb_system_tick =
        callbacks.get_key("system_tick")
                 .expect("system_tick key");
    let wlcb_game_tick =
        callbacks.get_key("game_tick")
                 .expect("game_tick key");
    let wlcb_init = callbacks.get_key("init").expect("init key");

    let wl_ctx_st = wl_ctx.clone();
    s_gs.borrow_mut().reg_cb("ship_tick".to_string(),
        move |gs: &Rc<RefCell<GameState>>, v: VVal| {
            let s = get_ship_state(&*gs.borrow(), v.i() as ObjectID).unwrap();
            let args = vec![s];
            if let Err(e) = wl_ctx_st.clone().call(&wlcb_ship_tick, &args) {
                println!("ERROR IN ship_tick: {}", e);
            }
        });

    let wl_ctx_st2 = wl_ctx.clone();
    s_gs.borrow_mut().reg_cb("system_tick".to_string(),
        move |gs: &Rc<RefCell<GameState>>, v: VVal| {
            let s = get_system_state(&*gs.borrow(), v.i() as ObjectID).unwrap();
            let args = vec![s];
            if let Err(e) = wl_ctx_st2.clone().call(&wlcb_system_tick, &args) {
                println!("ERROR IN system_tick: {}", e);
            }
        });

    let wl_ctx_st3 = wl_ctx.clone();
    s_gs.borrow_mut().reg_cb("system_tick".to_string(),
        move |gs: &Rc<RefCell<GameState>>, v: VVal| {
            let s = get_system_state(&*gs.borrow(), v.i() as ObjectID).unwrap();
            let args = vec![s];
            if let Err(e) = wl_ctx_st3.clone().call(&wlcb_game_tick, &args) {
                println!("ERROR IN game_tick: {}", e);
            }
        });

    {
        let ship = {
            let mut gs = s_gs.borrow_mut();
            let s = {
                let mut os = gs.object_registry.borrow_mut();
                os.add_ship(Ship::new("Cocky".to_string()))
            };
            gs.active_ship_id = s.borrow().id;
            s
        };
        let args = vec![ShipWlWrapper::vval_from(ship.clone())];
        if let Err(e) = wl_ctx.call(&wlcb_init, &args) {
            println!("ERROR IN init: {}", e);
        }
    }

//    let mut test_win = gui::Window::new();
//    test_win.x = 0;
//    test_win.y = 500;
//    test_win.w = 250;
//    test_win.h = 500;
//    test_win.title = String::from("Test 123");
//    let id = test_win.add_label(
//        gui::Size { w: 200, h: 0, min_w: 0, min_h: 0, margin: 0 },
//        gui::Label::new("TextLabel", (255, 255, 0, 255), (0, 128, 0, 255))
//        .center().wrap().lblref("XX1"));
//    let id2 = test_win.add_label(
//        gui::Size { w: 200, h: 0, min_w: 0, min_h: 0, margin: 0 },
//        gui::Label::new("TextLabel", (255, 255, 0, 255), (0, 128, 0, 255))
//        .center().wrap().lblref("XX2"));
//    let lay = test_win.add_layout(
//        gui::Size { w: 1000, h: 1000, min_w: 0, min_h: 0, margin: 0 },
//        gui::BoxDir::Vert(10),
//        &vec![id, id2]);
//    let id3 = test_win.add_label(
//        gui::Size { w: 200, h: 0, min_w: 200, min_h: 0, margin: 0 },
//        gui::Label::new("TextLabel", (255, 255, 0, 255), (0, 128, 0, 255))
//        .center().wrap().lblref("OF"));
//    let lay2 = test_win.add_layout(
//        gui::Size { w: 1000, h: 1000, min_w: 0, min_h: 0, margin: 0 },
//        gui::BoxDir::Vert(0),
//        &vec![lay, id3]);
//    test_win.child = lay2;

    let mut cb_queue : std::vec::Vec<(Rc<EventCallback>, VVal)> =
        std::vec::Vec::new();

    let mut last_frame = Instant::now();
    'running: loop {
        let active_ship_id = s_gs.borrow().active_ship_id;
        let active_ship    = s_gs.borrow().get_ship(active_ship_id)
                               .expect("active ship is present");
        let system_of_ship =
            s_gs.borrow_mut().get_system(active_ship.borrow().system);

        let mouse_state = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F2), .. } => {
                    let ser = s_gs.borrow().serialize();
                    if let Err(e) = util::write_file_safely(
                            "sscg_save.json", &ser.to_json(false).unwrap()) {
                        println!("FAILED TO WRITE SAVEFILE: {}", e);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
                    let ser = util::read_vval_json_file("sscg_save.json");
                    s_gs.borrow_mut().deserialize(ser);
                },
//                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
//                    fm.process_page_control(PageControl::CursorDown, None);
//                },
//                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
//                    fm.process_page_control(PageControl::CursorUp, None);
//                },
//                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
//                    fm.process_page_control(PageControl::Access, None);
//                },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    for w in s_wm.borrow_mut().windows.iter_mut() {
                        if let Some(w) = w {
                            w.handle_event(gui::WindowEvent::Backspace);
                        }
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    for w in s_wm.borrow_mut().windows.iter_mut() {
                        if let Some(w) = w {
                            w.handle_event(gui::WindowEvent::MousePos(x, y));
                        }
                    }
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if let Some(sys) = system_of_ship.clone() {
                        if let Some(e) = sys.borrow_mut().get_entity_close_to_screen(x, y) {
                            let x = e.borrow().x;
                            let y = e.borrow().y;
                            active_ship.borrow_mut().set_course_to(x, y);
                        }
                    }

                    for w in s_wm.borrow_mut().windows.iter_mut() {
                        if let Some(w) = w {
                            w.handle_event(gui::WindowEvent::Click(x, y));
                        }
                    }
                },
                Event::TextInput { text, .. } => {
                    for w in s_wm.borrow_mut().windows.iter_mut() {
                        if let Some(w) = w {
                            w.handle_event(gui::WindowEvent::TextInput(text.clone()));
                        }
                    }
                },
                Event::Window { win_event: w, timestamp: _, window_id: _ } => {
                    match w {
                        WindowEvent::Resized(w, h) => {
                            println!("XHX {},{}", w, h);
//                            fm.handle_resize();
                        },
                        WindowEvent::SizeChanged(w, h) => {
                            println!("XHXSC {},{}", w, h);
//                            fm.handle_resize();
                        },
                        WindowEvent::FocusGained => {
                        },
                        WindowEvent::FocusLost => {
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        s_wm.borrow_mut().handle_activated_childs(&mut wl_ctx);

        let active_ship_id = s_gs.borrow().active_ship_id;
        let active_ship    = s_gs.borrow().get_ship(active_ship_id)
                               .expect("active ship is present");
        let system_of_ship =
            s_gs.borrow_mut().get_system(active_ship.borrow().system);

        let frame_time_ms = last_frame.elapsed().as_micros() as f64 / 1000.0_f64;
        s_gs.borrow_mut().update(frame_time_ms);
        s_gs.borrow_mut().event_router.borrow_mut().get_events(&mut cb_queue);

        while !cb_queue.is_empty() {
            let c = cb_queue.pop().unwrap();
            c.0(&s_gs, c.1);
        }

        sdl_painter.clear();
        {
            if let Some(sys) = system_of_ship {
                sys.borrow_mut().draw(
                    &mut *active_ship.borrow_mut(),
                    &mut sdl_painter);
                sys.borrow_mut()
                   .try_highlight_entity_close_to(
                        mouse_state.x(),
                        mouse_state.y());
            }
        }
        let win_size = sdl_painter.canvas.window().size();
        for w in s_wm.borrow_mut().windows.iter_mut() {
            if let Some(w) = w {
                w.draw(win_size.0, win_size.1, &mut sdl_painter);
            }
        }
        sdl_painter.done();
        last_frame = Instant::now();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
