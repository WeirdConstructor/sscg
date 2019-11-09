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
mod tree_painter;
mod wlambda_api;
use wlambda_api::*;
use logic::*;
use tree_painter::*;

use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};

pub fn draw_cmds<'a>(
    cmds:      &[DrawCmd],
    canvas:    &mut sdl2::render::Canvas<sdl2::video::Window>,
    txt_crt:   &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font:      &sdl2::ttf::Font,
    txt_cache: &mut std::vec::Vec<Option<Rc<sdl2::render::Texture<'a>>>>,
    textures:  &std::vec::Vec<sdl2::render::Texture>) {

    use sdl2::gfx::primitives::{DrawRenderer};

    for c in cmds {
//        println!("CMD {:?}", c);
        match c {
            DrawCmd::CacheDraw { w, h, id, cmds } => {
                if *id >= txt_cache.len() {
                    txt_cache.resize(*id + 1, None);
                }
                let mut t =
                    txt_crt.create_texture_target(
                        sdl2::pixels::PixelFormatEnum::RGBA8888,
                        *w, *h).unwrap();
                t.set_blend_mode(sdl2::render::BlendMode::Blend);

                canvas.with_texture_canvas(&mut t, |mut canvas| {
                    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                    canvas.clear();
                    draw_cmds(&cmds, &mut canvas, txt_crt,
                              font,
                              txt_cache,
                              textures);
                });
                txt_cache[*id] = Some(Rc::new(t));
            },
            DrawCmd::DrawCache { x, y, w, h, id } => {
                if let Some(t) = &txt_cache[*id] {
//                    println!("DRAW {}, {}, {}, {}", *x, *y, *w, *h);
                    canvas.copy(
                        &t,
                        Some(Rect::new(0,   0, *w, *h)),
                        Some(Rect::new(*x, *y, *w, *h))
                    ).map_err(|e| e.to_string()).unwrap();
                }
            },
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
            DrawCmd::TextureCrop { txt_idx, x, y, w, h } => {
                if *txt_idx >= textures.len() { return; }
                if let Some(t) = textures.get(*txt_idx) {
                    let q = t.query();
                    let mut w = *w;
                    let mut h = *h;
                    if q.width  < w { w = q.width; }
                    if q.height < h { h = q.height; }
                    canvas.copy(
                        t,
                        Some(Rect::new(0, 0,   w, h)),
                        Some(Rect::new(*x, *y, w, h)));
                }
            },
            DrawCmd::Texture { txt_idx, x, y, centered } => {
                if *txt_idx >= textures.len() { return; }
                if let Some(t) = textures.get(*txt_idx) {
                    let q = t.query();
                    let mut rx : i32 = 0;
                    let mut ry : i32 = 0;
                    if *centered {
                        rx = -(q.width as i32 / 2);
                        ry = -(q.height as i32 / 2);
                    }
                    canvas.copy(
                        t,
                        Some(Rect::new(0, 0, q.width, q.height)),
                        Some(Rect::new(x + rx, y + ry, q.width, q.height)));
                }
            },
            DrawCmd::Circle { x, y, r, color } => {
                canvas.circle(*x as i16, *y as i16, *r as i16,
                    Color::from(*color)).expect("drawing circle");
            },
            DrawCmd::FilledCircle { x, y, r, color } => {
                canvas.filled_circle(*x as i16, *y as i16, *r as i16,
                    Color::from(*color)).expect("drawing circle");
            },
            DrawCmd::Line { x, y, x2, y2, t, color } => {
                canvas.thick_line(
                    *x as i16,
                    *y as i16,
                    *x2 as i16,
                    *y2 as i16,
                    *t as u8,
                    Color::from(*color))
                    .expect("drawing thick_line");
            },
            DrawCmd::Rect { x, y, w, h, color } => {
                canvas.set_draw_color(Color::from(*color));
                canvas.draw_rect(Rect::new(*x, *y, *w, *h))
                    .expect("drawing rectangle");
            },
            DrawCmd::FilledRect { x, y, w, h, color } => {
                canvas.set_draw_color(Color::from(*color));
                canvas.fill_rect(Rect::new(*x, *y, *w, *h))
                    .expect("drawing rectangle");
            },
            DrawCmd::ClipRectOff => {
                canvas.set_clip_rect(None);
            },
            DrawCmd::ClipRect { x, y, w, h } => {
                canvas.set_clip_rect(Rect::new(*x, *y, *w, *h));
            },
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
    font.set_style(sdl2::ttf::FontStyle::BOLD);// | sdl2::ttf::FontStyle::UNDERLINE);
    font.set_hinting(sdl2::ttf::Hinting::Normal);
//    font.set_outline_width(0.1);
    font.set_kerning(true);

    let tc = canvas.texture_creator();

    let mut asset_textures = std::vec::Vec::new();
    let t = tc.load_texture(std::path::Path::new("assets/images/models/rocks/asteroid_1_0001.png"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed textures"));
    }
    asset_textures.push(t.unwrap());

    let t = tc.load_texture(std::path::Path::new("assets/images/Orion_Nebula_-_Hubble_2006_mosaic_1800.jpg"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed asset_textures"));
    }
    asset_textures.push(t.unwrap());

    let t = tc.load_texture(std::path::Path::new("assets/images/models/stations/station_1_0001.png"));
    if let Err(e) = t {
        eprintln!("Couldn't load texture: {}", "test.png");
        return Err(String::from("failed asset_textures"));
    }
    asset_textures.push(t.unwrap());

    for i in 1..9 {
        let path = String::from("assets/images/models/ships/ship_1_000") + &i.to_string() + ".png";
        let t = tc.load_texture(std::path::Path::new(&path));
        if let Err(e) = t {
            eprintln!("Couldn't load texture: {}", "test.png");
            return Err(String::from("failed asset_textures"));
        }
        asset_textures.push(t.unwrap());
    }


    let cls = |idx: usize, xo: i32, yo: i32, w: u32, h: u32| {
    };
    let img_ctx = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let font = Rc::new(RefCell::new(font));

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

    let mut map_txt_cache : std::vec::Vec<Option<Rc<sdl2::render::Texture>>> =
        std::vec::Vec::new();
    let mut win_txt_cache : std::vec::Vec<Option<Rc<sdl2::render::Texture>>> =
        std::vec::Vec::new();

    let mut system_scroll : (i32, i32) = (0, 0);
    let mut last_mssp = MouseScreenSystemPos::new();

    {
        let font2 = font.clone();
        let mut win_tree_painter =
            TreePainter::new(|txt: &str| {
                font.borrow().size_of(txt).unwrap_or((0, 0))
            }, |idx: usize| {
                let tq = asset_textures[idx].query();
                (tq.width, tq.height)
            });

        let mut map_tree_painter =
            TreePainter::new(|txt: &str| {
                font2.borrow().size_of(txt).unwrap_or((0, 0))
            }, |idx: usize| {
                let tq = asset_textures[idx].query();
                (tq.width, tq.height)
            });

        let mut last_frame = Instant::now();
        'running: loop {
            let mut frame_time = Instant::now();

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
                    Event::Unknown { type_, timestamp } => {
                        if type_ == 8192 || type_ == 8193 {
                            for w in s_wm.borrow_mut().windows.iter_mut() {
                                if let Some(w) = w { w.does_need_redraw(); }
                            }
                            s_gs.borrow().object_registry.borrow_mut().all_entities_need_redraw();
                        }
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
                        let mut handled = false;
                        for w in s_wm.borrow_mut().windows.iter_mut() {
                            if let Some(w) = w {
                                if w.handle_event(gui::WindowEvent::Click(x, y)) {
                                    handled = true;
                                }
                            }
                        }

                        if !handled {
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

            let keys : Vec<Keycode> =
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

            let acts = s_wm.borrow_mut().get_activated_childs();
            if let Some(acts) = acts {
                for (idx, lblref, cb) in acts {
                    let args = vec![
                        VVal::Int(idx as i64),
                        VVal::new_str_mv(lblref)
                    ];
                    if let Err(e) = wl_ctx.clone().call(&cb, &args) {
                        println!("ERROR IN WM CB: {}", e);
                    }
                }
            }

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

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            let win_size = canvas.window().size();
            {
                map_tree_painter.clear_cmds();

                if win_size.0 > 1280 {
                    map_tree_painter.push_offs(((win_size.0 - 1280) / 2) as i32, 0);
                } else {
                    map_tree_painter.push_offs(0, 0);
                }

                if let Some(sys) = system_of_ship {
                    last_mssp =
                        sys.borrow_mut().draw(
                            &mut *active_ship.borrow_mut(),
                            &system_scroll,
                            &mut map_tree_painter);
                    sys.borrow_mut()
                       .try_highlight_entity_close_to(
                            mouse_state.x(),
                            mouse_state.y());
                }

                draw_cmds(map_tree_painter.ref_cmds(), &mut canvas,
                          &tc, &*font.borrow(),
                          &mut map_txt_cache,
                          &asset_textures);
                map_tree_painter.pop_offs();
            }
            win_tree_painter.clear_cmds();
            for (i, w) in s_wm.borrow_mut().windows.iter_mut().enumerate() {
                if let Some(w) = w {
                    w.draw(i, win_size.0, win_size.1, &mut win_tree_painter);
                }
            }
            draw_cmds(win_tree_painter.ref_cmds(), &mut canvas,
                      &tc, &*font.borrow(),
                      &mut win_txt_cache,
                      &asset_textures);

            canvas.present();
            last_frame = Instant::now();

            //d// println!("FRAME MS: {}", last_frame.elapsed().as_nanos() as f64 / 1000_f64);

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    Ok(())
}
