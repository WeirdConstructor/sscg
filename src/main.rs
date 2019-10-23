use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
//use sdl2::rect::Point;
use sdl2::gfx::primitives::{DrawRenderer};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};
//use vecmath::*;

mod logic;
mod util;
mod gui;
use logic::*;

use wlambda::{VVal, StackAction, VValUserData, GlobalEnv, EvalContext};

struct GUIPainter<'a, 'b> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: Rc<RefCell<sdl2::ttf::Font<'a, 'b>>>,
    font_h: i32,
    offs_stack: std::vec::Vec<(i32, i32)>,
    offs: (i32, i32),
}

impl<'a, 'b> GUIPainter<'a, 'b> {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    fn get_font_h(&mut self) -> i32 {
        if self.font_h == 0 {
            let (w, h) = self.text_size("M");
            self.font_h = h as i32;
        }
        self.font_h
    }

    fn text_size(&mut self, txt: &str) -> (u32, u32) {
        self.font.borrow().size_of(txt).unwrap_or((0, 0))
    }

    fn done(&mut self) {
        self.canvas.present();
    }
}

impl<'a, 'b> GamePainter for GUIPainter<'a, 'b> {
    fn push_offs(&mut self, xo: i32, yo: i32) {
        self.offs_stack.push(self.offs);
        self.offs = (xo, yo);
    }

    fn push_add_offs(&mut self, xo: i32, yo: i32) {
        self.push_offs(xo + self.offs.0, yo + self.offs.1);
    }

    fn pop_offs(&mut self) {
        if !self.offs_stack.is_empty() {
            self.offs = self.offs_stack.pop().unwrap();
        }
    }

    fn get_screen_pos(&self, xo: i32, yo: i32) -> (i32, i32) {
        ((self.offs.0 + xo) as i32,
         (self.offs.1 + yo) as i32)
    }

    fn get_sprite_size(&self, _id: usize) -> (u32, u32) {
        (0, 0)
    }
    fn draw_sprite_ex(&mut self, _xo: i32, _yo: i32, _w: u32, _h: u32,
                      _id: usize, _angle: f64, _flip_h: bool, _flip_v: bool) {
    }

    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8)) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.draw_rect(Rect::new(xo + self.offs.0, yo + self.offs.1, w, h))
            .expect("drawing rectangle");
    }

    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8)) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.fill_rect(Rect::new(xo + self.offs.0, yo + self.offs.1, w, h))
            .expect("filling rectangle");
    }

    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.canvas.filled_circle(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            r as i16,
            Color::from(color));
    }
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.canvas.circle(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            r as i16,
            Color::from(color));
    }
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32, color: (u8, u8, u8, u8)) {
        self.canvas.thick_line(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            (self.offs.0 + x2o) as i16,
            (self.offs.1 + y2o) as i16,
            t as u8,
            Color::from(color));
    }
    fn text_size(&mut self, txt: &str) -> (u32, u32) {
        self.text_size(txt)
    }

    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32, fg: (u8, u8, u8, u8),
                 bg: Option<(u8, u8, u8, u8)>, align: i32, txt: &str) {
        if let Some(c) = bg {
            let h = self.get_font_h();
            draw_bg_text(
                &mut self.canvas,
                &mut *self.font.borrow_mut(),
                fg.into(),
                c.into(),
                self.offs.0 + xo,
                self.offs.1 + yo,
                max_w as i32,
                h,
                align,
                txt);
        } else {
            draw_text(
                &mut *self.font.borrow_mut(),
                fg.into(),
                &mut self.canvas,
                self.offs.0 + xo,
                self.offs.1 + yo,
                max_w as i32,
                align,
                txt);
        }
    }
}

fn draw_text(font: &mut sdl2::ttf::Font, color: Color,
             canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
             x: i32, y: i32, max_w: i32, align: i32, txt: &str) {
    let txt_crt = canvas.texture_creator();

    let sf = font.render(txt).blended(color).map_err(|e| e.to_string()).unwrap();
    let txt = txt_crt.create_texture_from_surface(&sf).map_err(|e| e.to_string()).unwrap();
    let tq = txt.query();

    let xo = if align == 0 { (max_w - (tq.width as i32)) / 2 }
        else if align < 0  { max_w - (tq.width as i32) }
        else { 0 };

    let w : i32 = if max_w < (tq.width as i32) { max_w } else { tq.width as i32 };

//    txt.set_color_mod(255, 0, 0);
    canvas.copy(
        &txt,
        Some(Rect::new(0, 0, w as u32, tq.height)),
        Some(Rect::new(x + xo, y, w as u32, tq.height))
    ).map_err(|e| e.to_string()).unwrap();
}

fn draw_bg_text(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
                font: &mut sdl2::ttf::Font,
                color: Color,
                bg_color: Color,
                x: i32,
                y: i32,
                max_w: i32,
                h: i32,
                align: i32,
                txt: &str) {

    canvas.set_draw_color(bg_color);
    canvas.fill_rect(Rect::new(x, y, max_w as u32, h as u32))
        .expect("filling rectangle");
    draw_text(font, color, canvas, x, y, max_w, align, txt);
}

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

    let mut gui_painter = GUIPainter {
        canvas: canvas,
        font: Rc::new(RefCell::new(font)),
        font_h: 0,
        offs_stack: std::vec::Vec::new(),
        offs: (0, 0),
    };

    let GS = GameState::new_ref();

    let genv = GlobalEnv::new_default();
    let mut wl_ctx = EvalContext::new_with_user(genv, GS.clone());

    wl_ctx.set_global_var("game", &GameStateWlWrapper::vval_from(GS.clone()));

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

    let wlcb_ship_ent_tick =
        callbacks.get_key("ship_entity_tick")
                 .expect("ship_entity_tick key");
    let wlcb_ship_tick =
        callbacks.get_key("ship_tick")
                 .expect("ship_tick key");
//    let wlcb_system_tick   = callbacks.get_key("system_tick");
//    let wlcb_ship_arrived  = callbacks.get_key("ship_arrived");
    let wlcb_init          = callbacks.get_key("init").expect("init key");

    let wl_ctx_st = wl_ctx.clone();
    GS.borrow_mut().reg_cb("ship_tick".to_string(),
        move |gs: &Rc<RefCell<GameState>>, v: VVal| {
            let ship   : Rc<RefCell<Ship>>   = gs.borrow().get_ship(v.i() as ObjectID).unwrap();
            let system : Rc<RefCell<System>> = gs.borrow().get_system(ship.borrow().system).unwrap();

            let v_ship = ShipWlWrapper::vval_from(ship.clone());
            let v_sys_id  = VVal::Int(ship.borrow().system as i64);
            {
                let args = vec![v_ship.clone().into(), v_sys_id.clone()];
                wl_ctx_st.clone().call(&wlcb_ship_tick, &args);
            }

            let e = system.borrow_mut().get_entity_close_to(
                        ship.borrow().pos.0, ship.borrow().pos.1);
            if let Some(ent) = e {
                println!("SHIP ARRIVED: {} AT SYS {} ENT: {:?}",
                    v.s(), system.borrow().id, *(ent.borrow()));
                let typ = VVal::new_str(
                    match ent.borrow().typ {
                        SystemObject::Station       => "station",
                        SystemObject::AsteroidField => "asteroid_field",
                    }
                );
                let v_ent = EntityWlWrapper::vval_from(ent.clone());
                let args : Vec<VVal> = vec![
                    v_ship.into(),
                    v_sys_id,
                    v_ent.into(),
                    typ,
                ];
                wl_ctx_st.clone().call(&wlcb_ship_ent_tick, &args);
            }
        });

    GS.borrow_mut().reg_cb("ship_arrived".to_string(),
        |gs: &Rc<RefCell<GameState>>, v: VVal| {
            let ship   : Rc<RefCell<Ship>>   =
                gs.borrow().get_ship(v.i() as ObjectID).unwrap();
            let system : Rc<RefCell<System>> =
                gs.borrow().get_system(ship.borrow().system).unwrap();
            let e = system.borrow_mut().get_entity_close_to(
                        ship.borrow().pos.0, ship.borrow().pos.1);
            if let Some(ent) = e {
                println!("SHIP ARRIVED: {} AT SYS {} ENT: {:?}", v.s(), system.borrow().id, *(ent.borrow()));
            }
        });

    {
        let ship = {
            let mut gs = GS.borrow_mut();
            let s = {
                let mut os = gs.object_registry.borrow_mut();
                os.add_ship(Ship::new("Cocky".to_string()))
            };
            gs.active_ship_id = s.borrow().id;
            s
        };
        let args = vec![ShipWlWrapper::vval_from(ship.clone())];
        wl_ctx.clone().call(&wlcb_init, &args);
    }

    let mut test_win = gui::Window::new();
    test_win.x = 0;
    test_win.y = 500;
    test_win.w = 250;
    test_win.h = 500;
    test_win.title = String::from("Test");
    let id = test_win.add_label(
        gui::Size { w: 200, h: 600, min_w: 0, min_h: 0 },
        gui::Label::new("TextLabel", (255, 255, 0, 255), (0, 128, 0, 255))
        .center().wrap());
    test_win.child = id;

    let mut cb_queue : std::vec::Vec<(Rc<EventCallback>, VVal)> = std::vec::Vec::new();

    let mut last_frame = Instant::now();
    'running: loop {
        let active_ship_id = GS.borrow().active_ship_id;
        let active_ship    = GS.borrow().get_ship(active_ship_id)
                               .expect("active ship is present");
        let system_of_ship = GS.borrow_mut().get_system(active_ship.borrow().system);

        let mouse_state = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::F2), .. } => {
                    let ser = GS.borrow().serialize();
                    util::write_file_safely("sscg_save.json", &ser.to_json(false).unwrap());
                },
                Event::KeyDown { keycode: Some(Keycode::F3), .. } => {
                    let ser = util::read_vval_json_file("sscg_save.json");
                    GS.borrow_mut().deserialize(ser);
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
                    test_win.handle_event(gui::WindowEvent::Backspace);
                },
                Event::MouseMotion { x, y, .. } => {
                    test_win.handle_event(gui::WindowEvent::MousePos(x, y));
                },
                Event::MouseButtonDown { x, y, .. } => {
                    if let Some(sys) = system_of_ship.clone() {
                        if let Some(e) = sys.borrow_mut().get_entity_close_to(x, y) {
                            let x = e.borrow().x;
                            let y = e.borrow().y;
                            active_ship.borrow_mut().set_course_to(x, y);
                        }
                    }

                    test_win.handle_event(gui::WindowEvent::Click(x, y));
                },
                Event::TextInput { text, .. } => {
                    test_win.handle_event(gui::WindowEvent::TextInput(text.clone()));
                    println!("TEXT: {}", text);
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

        let active_ship_id = GS.borrow().active_ship_id;
        let active_ship    = GS.borrow().get_ship(active_ship_id)
                               .expect("active ship is present");
        let system_of_ship = GS.borrow_mut().get_system(active_ship.borrow().system);

        let frame_time_ms = last_frame.elapsed().as_micros() as f64 / 1000.0_f64;
        GS.borrow_mut().update(frame_time_ms);
        GS.borrow_mut().event_router.borrow_mut().get_events(&mut cb_queue);

        while !cb_queue.is_empty() {
            let c = cb_queue.pop().unwrap();
            c.0(&GS, c.1);
        }

        gui_painter.clear();
        {
            if let Some(sys) = system_of_ship {
                sys.borrow_mut().draw(&mut *active_ship.borrow_mut(), &mut gui_painter);
                sys.borrow_mut()
                   .try_highlight_entity_close_to(
                        mouse_state.x(),
                        mouse_state.y());
            }
        }
        let win_size = gui_painter.canvas.window().size();
        test_win.draw(win_size.0, win_size.1, &mut gui_painter);
        gui_painter.done();
        last_frame = Instant::now();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
