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
use logic::*;

use wlambda::{VVal}; //, GlobalEnv, EvalContext};

struct GUIPainter<'a, 'b> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: Rc<RefCell<sdl2::ttf::Font<'a, 'b>>>,
    offs_stack: std::vec::Vec<(i32, i32)>,
    offs: (i32, i32),
}

impl<'a, 'b> GUIPainter<'a, 'b> {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
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
    fn draw_text(&mut self, _xo: i32, _yo: i32, _max_w: u32, _fg: (u8, u8, u8, u8), _bg: Option<(u8, u8, u8, u8)>, _txt: &str) {
    }
}

fn draw_text(font: &mut sdl2::ttf::Font, color: Color, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, max_w: i32, txt: &str) {
    let txt_crt = canvas.texture_creator();

    let sf = font.render(txt).blended(color).map_err(|e| e.to_string()).unwrap();
    let txt = txt_crt.create_texture_from_surface(&sf).map_err(|e| e.to_string()).unwrap();
    let tq = txt.query();

    let w : i32 = if max_w < (tq.width as i32) { max_w } else { tq.width as i32 };

//    txt.set_color_mod(255, 0, 0);
    canvas.copy(
        &txt,
        Some(Rect::new(0, 0, w as u32, tq.height)),
        Some(Rect::new(x, y, w as u32, tq.height))
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
                txt: &str) {

    canvas.set_draw_color(bg_color);
    canvas.fill_rect(Rect::new(x, y, max_w as u32, h as u32))
        .expect("filling rectangle");
    draw_text(font, color, canvas, x, y, max_w, txt);
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
        offs_stack: std::vec::Vec::new(),
        offs: (0, 0),
    };

    let OReg = ObjectRegistry::new();
    let ERouter = EventRouter::new();
    let mut GS = GameState {
        object_registry: Rc::new(RefCell::new(OReg)),
        event_router:    Rc::new(RefCell::new(ERouter)),
    };

    GS.event_router.borrow_mut().reg_cb("ship_arrived".to_string(),
        |gs: &mut GameState, v: VVal| {
            let ship   : Rc<RefCell<Ship>>   = gs.get_ship(v.i() as ObjectID).unwrap();
            let system : Rc<RefCell<System>> = gs.get_system(ship.borrow().system).unwrap();
            let e = system.borrow_mut().get_entity_close_to(ship.borrow().pos.0, ship.borrow().pos.1);
            if let Some(ent) = e {
                println!("SHIP ARRIVED: {} AT SYS {} ENT: {:?}", v.s(), system.borrow().id, *(ent.borrow()));
            }
        });

    let (s, ship) = {
        let mut os = GS.object_registry.borrow_mut();
        let s = os.add_system(System::new(0, 0));
        s.borrow_mut().add(10,   10, os.add_entity(Entity::new(logic::SystemObject::Station)));
        s.borrow_mut().add(400, 200, os.add_entity(Entity::new(logic::SystemObject::AsteroidField)));
        s.borrow_mut().add(150, 300, os.add_entity(Entity::new(logic::SystemObject::AsteroidField)));

        let ship = os.add_ship(Ship::new("Cocky".to_string(), 100, 100));
        ship.borrow_mut().set_system(s.borrow().id);
        (s, ship)
    };

    let mut cb_queue : std::vec::Vec<(Rc<EventCallback>, VVal)> = std::vec::Vec::new();

    let mut last_frame = Instant::now();
    'running: loop {
        let mouse_state = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
//                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
//                    fm.process_page_control(PageControl::Back, None);
//                },
//                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
//                    fm.process_page_control(PageControl::CursorDown, None);
//                },
//                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
//                    fm.process_page_control(PageControl::CursorUp, None);
//                },
//                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
//                    fm.process_page_control(PageControl::Access, None);
//                },
                Event::MouseButtonDown { x, y, .. } => {
                    if let Some(e) = s.borrow_mut().get_entity_close_to(x, y) {
                        let x = e.borrow().x;
                        let y = e.borrow().y;
                        ship.borrow_mut().set_course_to(x, y);
                    }
                },
//                Event::TextInput { text, .. } => {
//                    println!("TEXT: {}", text);
//                },
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

        let frame_time_ms = last_frame.elapsed().as_micros() as f64 / 1000.0_f64;
        {
            let mut os = GS.object_registry.borrow_mut();
            os.update(frame_time_ms, &mut *(GS.event_router.borrow_mut()));
        }

        GS.event_router.borrow_mut().get_events(&mut cb_queue);

        while !cb_queue.is_empty() {
            let c = cb_queue.pop().unwrap();
            c.0(&mut GS, c.1);
        }

        gui_painter.clear();
        s.borrow_mut().draw(&mut *ship.borrow_mut(), &mut gui_painter);
        s.borrow_mut().try_highlight_entity_close_to(mouse_state.x(), mouse_state.y());
        gui_painter.done();
        last_frame = Instant::now();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
