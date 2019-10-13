use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Instant};

use wlambda;
use wlambda::{VVal, GlobalEnv, EvalContext};

struct GUIPainter<'a, 'b> {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    font: Rc<RefCell<sdl2::ttf::Font<'a, 'b>>>,
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
    };

    let mut last_frame = Instant::now();
    'running: loop {
        let mut force_redraw = false;
        let event = event_pump.wait_event_timeout(1);
        let mouse_state = event_pump.mouse_state();
        if let Some(event) = event {
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
//                Event::MouseButtonDown { x, y, .. } => {
//                    fm.process_page_control(PageControl::Click((x, y)), Some((x, y)));
//                },
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

            let frame_time = last_frame.elapsed().as_millis();

                gui_painter.clear();
//                fm.redraw(&mut gui_painter);
                gui_painter.done();
//                last_frame = Instant::now();
        }
    }

    Ok(())
}
