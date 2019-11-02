use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::image::{LoadTexture, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant, Duration};
//use vecmath::*;

mod logic;
mod util;
mod gui;
mod sdl_painter;
mod tree_painter;
mod wlambda_api;
use wlambda_api::*;
use logic::*;
use tree_painter::*;
use sdl_painter::SDLPainter;

use wlambda::{VVal, StackAction, VValUserData, GlobalEnv, EvalContext, SymbolTable};

pub fn draw_cmds(
    cmds:     &[DrawCmd],
    canvas:   &mut sdl2::render::Canvas<sdl2::video::Window>,
    txt_crt:  &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font:     &sdl2::ttf::Font) {

    for c in cmds {
        match c {
            DrawCmd::Text { txt, align, color, x, y, w } => {
                if txt.is_empty() { continue; }
                let max_w = *w as i32;
                let c : Color = (*color).into();
                let f =
                    font.render(txt).blended(c).map_err(|e| e.to_string()).unwrap();
                let txt = txt_crt.create_texture_from_surface(&f).map_err(|e| e.to_string()).unwrap();
                let tq  = txt.query();

                let xo = if *align == 2
                         || *align == 0 { (max_w - (tq.width as i32)) / 2 }
                    else if *align < 0  { max_w - (tq.width as i32) }
                    else { 0 };

                let w : i32 = if max_w < (tq.width as i32) { max_w } else { tq.width as i32 };

                let xo = if xo < 0 { 0 } else { xo };

                canvas.copy(
                    &txt,
                    Some(Rect::new(0,      0, w as u32, tq.height)),
                    Some(Rect::new(*x + xo, *y, w as u32, tq.height))
                ).map_err(|e| e.to_string()).unwrap();
            },
            _ => (),
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SSCG - Game", 1280, 720)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;

//    let mut font = ttf_ctx.load_font("DejaVuSansMono.ttf", 14).map_err(|e| e.to_string())?;
    let mut font = ttf_ctx.load_font("DejaVuSans-Bold.ttf", 14).map_err(|e| e.to_string())?;
    let mut font2 = ttf_ctx.load_font("DejaVuSans-Bold.ttf", 14).map_err(|e| e.to_string())?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);// | sdl2::ttf::FontStyle::UNDERLINE);
    font.set_hinting(sdl2::ttf::Hinting::Normal);
//    font.set_outline_width(0.1);
    font.set_kerning(true);

    let tc = canvas.texture_creator();
    let mut textures = std::vec::Vec::new();

    let t = tc.load_texture(std::path::Path::new("assets/images/models/rocks/asteroid_1_0001.png"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed textures"));
    }
    textures.push(t.unwrap());

    let t = tc.load_texture(std::path::Path::new("assets/images/Orion_Nebula_-_Hubble_2006_mosaic_1800.jpg"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed textures"));
    }
    textures.push(t.unwrap());

    let t = tc.load_texture(std::path::Path::new("assets/images/models/stations/station_1_0001.png"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed textures"));
    }
    textures.push(t.unwrap());

    for i in 1..9 {
        let path = String::from("assets/images/models/ships/ship_1_000") + &i.to_string() + ".png";
        let t = tc.load_texture(std::path::Path::new(&path));
        if let Err(e) = t {
            eprintln!("Couldn't load texture: {}", "test.png");
            return Err(String::from("failed textures"));
        }
        textures.push(t.unwrap());
    }

    let cls = |idx: usize, xo: i32, yo: i32, w: u32, h: u32| {
    };
    let img_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let font = Rc::new(RefCell::new(font));

    let mut sdl_painter = SDLPainter {
        canvas:     canvas,
        text_cache: std::collections::HashMap::new(),
        font:       font.clone(),
        font_h:     0,
        offs_stack: std::vec::Vec::new(),
        offs:       (0, 0),
        textures,
    };

    let s_wm = WindowManager::new_ref();
    let s_gs = GameState::new_ref();

    let genv = GlobalEnv::new_default();
    let mut sscg_wl_mod = SymbolTable::new();
    sscg_wl_mod.set("game", GameStateWlWrapper::vval_from(s_gs.clone()));
    sscg_wl_mod.set("win",  WindowManagerWlWrapper::vval_from(s_wm.clone()));
    genv.borrow_mut().set_module("sscg", sscg_wl_mod);

    let mut wl_ctx = EvalContext::new_with_user(genv, s_gs.clone());


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
    let wlcb_load =
        callbacks.get_key("game_load")
                 .expect("game_load key");
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

    let args = vec![];
    if let Err(e) = wl_ctx.clone().call(&wlcb_load, &args) {
        println!("ERROR IN game_load: {}", e);
    }

    let mut cb_queue : std::vec::Vec<(Rc<EventCallback>, VVal)> =
        std::vec::Vec::new();

    let mut txts : std::vec::Vec<sdl2::render::Texture> = std::vec::Vec::new();
    txts.push(tc.create_texture_target(sdl2::pixels::PixelFormatEnum::RGBA8888, 10, 10).unwrap());

    let mut system_scroll : (i32, i32) = (0, 0);
    let mut last_mssp = MouseScreenSystemPos::new();

    {
        let mut tree_painter =
            TreePainter::new(|txt: &str| {
                font.borrow().size_of(txt).unwrap_or((0, 0))
            });

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

                        let args = vec![];
                        if let Err(e) = wl_ctx.clone().call(&wlcb_load, &args) {
                            println!("ERROR IN game_load: {}", e);
                        }
                    },
    //                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
    //                },
    //                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
    //                },
    //                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
    //                },
    //                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
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
                            } else {
                                if let Some(p) = last_mssp.mouse2system(x, y) {
                                    active_ship.borrow_mut().set_course_to(p.0, p.1);
                                }
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

            let keys : Vec<Keycode>=
                event_pump
                    .keyboard_state()
                    .pressed_scancodes()
                    .filter_map(Keycode::from_scancode)
                    .collect();

            let mut x_speed = 0;
            let mut y_speed = 0;

            for k in keys.iter() {
                match k {
                    Keycode::W => { y_speed = -2 },
                    Keycode::S => { y_speed = 2  },
                    Keycode::A => { x_speed = -2 },
                    Keycode::D => { x_speed = 2  },
                    _ => (),
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

            system_scroll.0 += (x_speed as f64 * frame_time_ms).round() as i32;
            system_scroll.1 += (y_speed as f64 * frame_time_ms).round() as i32;
            if system_scroll.0 > 1000 { system_scroll.0 = 1000; }
            if system_scroll.0 < 0    { system_scroll.0 = 0; }
            if system_scroll.1 > 1000 { system_scroll.1 = 1000; }
            if system_scroll.1 < 0    { system_scroll.1 = 0; }

            while !cb_queue.is_empty() {
                let c = cb_queue.pop().unwrap();
                c.0(&s_gs, c.1);
            }

            sdl_painter.clear();
            let win_size = sdl_painter.canvas.window().size();
            {
                if win_size.0 > 1280 {
                    sdl_painter.push_offs(((win_size.0 - 1280) / 2) as i32, 0);
                } else {
                    sdl_painter.push_offs(0, 0);
                }

                if let Some(sys) = system_of_ship {
                    last_mssp =
                        sys.borrow_mut().draw(
                            &mut *active_ship.borrow_mut(),
                            &system_scroll,
                            &mut sdl_painter);
                    sys.borrow_mut()
                       .try_highlight_entity_close_to(
                            mouse_state.x(),
                            mouse_state.y());
                }
                sdl_painter.pop_offs();
            }
            txts[0] =
                tc.create_texture_target(
                    sdl2::pixels::PixelFormatEnum::RGBA8888,
                    win_size.0,
                    win_size.1).unwrap();
            txts[0].set_blend_mode(sdl2::render::BlendMode::Blend);
            for w in s_wm.borrow_mut().windows.iter_mut() {
                if let Some(w) = w {
                    w.draw(win_size.0, win_size.1, &mut tree_painter);
                }
            }

            let cmds = tree_painter.consume_cmds();
            sdl_painter.canvas.with_texture_canvas(&mut txts[0], |mut canvas| {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                canvas.clear();
//                        canvas.set_draw_color(Color::RGBA(255, 255, 0, 255));
//                        canvas.fill_rect(Rect::new(0, 0, 400, 400)).unwrap();
                draw_cmds(&cmds, &mut canvas, &tc, &*font.borrow());
            });
            sdl_painter.canvas.copy(
                &txts[0],
                Some(Rect::new(0, 0, win_size.0, win_size.1)),
                Some(Rect::new(0, 0, win_size.0, win_size.1))
            ).map_err(|e| e.to_string()).unwrap();

            sdl_painter.done();
            last_frame = Instant::now();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    Ok(())
}
