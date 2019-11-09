#[macro_use]
extern crate lazy_static;
extern crate gdnative;
use gdnative::*;

use euclid::rect;
use euclid::vec2;

use sscg::gui;
use sscg::logic;
use sscg::tree_painter::{DrawCmd, TreePainter};

use std::rc::Rc;
use std::cell::RefCell;

use std::sync::{Arc, Mutex, Condvar};


struct Fonts {
    main_font: DynamicFont,
}

unsafe impl Send for Fonts {
}

fn c2c(c: (u8, u8, u8, u8)) -> Color {
    Color::rgba(
        c.0 as f32 / 255.0,
        c.1 as f32 / 255.0,
        c.2 as f32 / 255.0,
        c.3 as f32 / 255.0)
}

struct SSCGState {
    b: bool,
    v: std::vec::Vec<DrawCmd>,
    fonts: Option<Fonts>,

//    tp: TreePainter<Box<dyn Fn(&str) -> (u32, u32)>, Box<dyn Fn(usize) -> (u32, u32)>>,
}

lazy_static! {
    static ref SSCG : Arc<Mutex<SSCGState>> =
        Arc::new(Mutex::new(SSCGState {
            b: true,
            v: std::vec::Vec::new(),
            fonts: None,
        }));
}

#[derive(NativeClass)]
#[inherit(gdnative::Node2D)]
#[user_data(user_data::MutexData<GUIPaintNode>)]
pub struct GUIPaintNode {
    cmds: std::vec::Vec<DrawCmd>,
    f: Fonts,
//    tp:     Rc<RefCell<TreePainter<Box<dyn Fn(&str) -> (u32, u32)>, Box<dyn Fn(usize) -> (u32, u32)>>>>,
}

//struct ThreadGUIState {
//    f: Fonts,
//    cmds: std::vec::Vec<DrawCmd>,
//}
//
//struct ThreadGUIExchange {
//    cv: Condvar,
//    mx: Mutex<Option<ThreadGUIState>>,
//}
//
//impl {
//}

fn draw_cmds(n: &mut Node2D, f: &Fonts, cmds: &std::vec::Vec<DrawCmd>) {
    for c in cmds {
        match c {
            DrawCmd::CacheDraw { w, h, id, cmds } => {
//                if *id >= txt_cache.len() {
//                    txt_cache.resize(*id + 1, None);
//                }
//                let mut t =
//                    txt_crt.create_texture_target(
//                        sdl2::pixels::PixelFormatEnum::RGBA8888,
//                        *w, *h).unwrap();
//                t.set_blend_mode(sdl2::render::BlendMode::Blend);
//
//                canvas.with_texture_canvas(&mut t, |mut canvas| {
//                    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
//                    canvas.clear();
//                    draw_cmds(&cmds, &mut canvas, txt_crt,
//                              font,
//                              txt_cache,
//                              textures);
//                });
//                txt_cache[*id] = Some(Rc::new(t));
            },
            DrawCmd::DrawCache { x, y, w, h, id } => {
//                if let Some(t) = &txt_cache[*id] {
////                    println!("DRAW {}, {}, {}, {}", *x, *y, *w, *h);
//                    canvas.copy(
//                        &t,
//                        Some(Rect::new(0,   0, *w, *h)),
//                        Some(Rect::new(*x, *y, *w, *h))
//                    ).map_err(|e| e.to_string()).unwrap();
//                }
            },
            DrawCmd::Text { txt, align, color, x, y, w } => {
//                if txt.is_empty() { continue; }
//                let max_w = *w as i32;
//                let c : Color = (*color).into();
//                let f =
//                    font.render(txt).blended(c).map_err(|e| e.to_string()).unwrap();
//                let txt = txt_crt.create_texture_from_surface(&f).map_err(|e| e.to_string()).unwrap();
//                let tq  = txt.query();
//
//                let xo = if *align == 2
//                         || *align == 0 { (max_w - (tq.width as i32)) / 2 }
//                    else if *align < 0  { max_w - (tq.width as i32) }
//                    else { 0 };
//
//                let w : i32 = if max_w < (tq.width as i32) { max_w } else { tq.width as i32 };
//
//                let xo = if xo < 0 { 0 } else { xo };
//
//                canvas.copy(
//                    &txt,
//                    Some(Rect::new(0,      0, w as u32, tq.height)),
//                    Some(Rect::new(*x + xo, *y, w as u32, tq.height))
//                ).map_err(|e| e.to_string()).unwrap();
            },
            DrawCmd::TextureCrop { txt_idx, x, y, w, h } => {
//                if *txt_idx >= textures.len() { return; }
//                if let Some(t) = textures.get(*txt_idx) {
//                    let q = t.query();
//                    let mut w = *w;
//                    let mut h = *h;
//                    if q.width  < w { w = q.width; }
//                    if q.height < h { h = q.height; }
//                    canvas.copy(
//                        t,
//                        Some(Rect::new(0, 0,   w, h)),
//                        Some(Rect::new(*x, *y, w, h)));
//                }
            },
            DrawCmd::Texture { txt_idx, x, y, centered } => {
//                if *txt_idx >= textures.len() { return; }
//                if let Some(t) = textures.get(*txt_idx) {
//                    let q = t.query();
//                    let mut rx : i32 = 0;
//                    let mut ry : i32 = 0;
//                    if *centered {
//                        rx = -(q.width as i32 / 2);
//                        ry = -(q.height as i32 / 2);
//                    }
//                    canvas.copy(
//                        t,
//                        Some(Rect::new(0, 0, q.width, q.height)),
//                        Some(Rect::new(x + rx, y + ry, q.width, q.height)));
//                }
            },
            DrawCmd::Circle { x, y, r, color } => {
//                canvas.circle(*x as i16, *y as i16, *r as i16,
//                    Color::from(*color)).expect("drawing circle");
            },
            DrawCmd::FilledCircle { x, y, r, color } => {
//                canvas.filled_circle(*x as i16, *y as i16, *r as i16,
//                    Color::from(*color)).expect("drawing circle");
            },
            DrawCmd::Line { x, y, x2, y2, t, color } => {
//                canvas.thick_line(
//                    *x as i16,
//                    *y as i16,
//                    *x2 as i16,
//                    *y2 as i16,
//                    *t as u8,
//                    Color::from(*color))
//                    .expect("drawing thick_line");
            },
            DrawCmd::Rect { x, y, w, h, color } => {
                unsafe {
                    n.draw_rect(
                        rect(*x as f32,
                             *y as f32,
                             *w as f32,
                             *h as f32),
                        c2c(*color),
                        true);
                }
//                canvas.set_draw_color(Color::from(*color));
//                canvas.draw_rect(Rect::new(*x, *y, *w, *h))
//                    .expect("drawing rectangle");
            },
            DrawCmd::FilledRect { x, y, w, h, color } => {
//                canvas.set_draw_color(Color::from(*color));
//                canvas.fill_rect(Rect::new(*x, *y, *w, *h))
//                    .expect("drawing rectangle");
            },
            DrawCmd::ClipRectOff => {
//                canvas.set_clip_rect(None);
            },
            DrawCmd::ClipRect { x, y, w, h } => {
//                canvas.set_clip_rect(Rect::new(*x, *y, *w, *h));
            },
        }
    }
}

#[methods]
impl GUIPaintNode {
    fn _init(_owner: Node2D) -> Self {
//        let tp =
//            Rc::new(RefCell::new(TreePainter::new(|txt: &str| {
//                (0, 0)
////                font2.borrow().size_of(txt).unwrap_or((0, 0))
//            }, |idx: usize| {
//                (0, 0)
////                let tq = asset_textures[idx].query();
////                (tq.width, tq.height)
//            })));

        let f =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://fonts/main_font_normal.tres"),
                GodotString::from_str("DynamicFont"),
                false);
        let df : DynamicFont = f.and_then(|f| f.cast::<DynamicFont>()).unwrap();
        GUIPaintNode {
            cmds: std::vec::Vec::new(),
            f: Fonts {
                main_font: df,
            },
        }
    }

    #[export]
    fn _ready(&self, _owner: Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("NODE PAINT READY");
    }

    #[export]
    fn _draw(&mut self, mut s: Node2D) {
        let mut d = SSCG.lock().unwrap();
        if !d.v.is_empty() {
            self.cmds = std::mem::replace(&mut d.v, std::vec::Vec::new());
            godot_print!("GOT IT");
        }

//        for v in self.cmds.iter() {
//            godot_print!("FO {:?}", v);
//        }

        draw_cmds(&mut s, &self.f, &self.cmds);

        unsafe {
            s.draw_string(
                Some(self.f.main_font.to_font()),
                vec2(50.0, 50.0),
                GodotString::from_str("BUKAKKE"),
                c2c((55, 0, 55, 255)),
                100);
//            godot_print!("DRAW: {} ", s.get_name().to_string());
//            s.draw_rect(rect(10.0, 10.0, 200.0, 200.0), Color::rgba(255.0, 1.0, 0.0, 255.0), true);
//            s.draw_circle(vec2(50.0, 50.0), 20.0, Color::rgb(1.0, 0.0, 1.0));
        }
    }
}


/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(gdnative::Node)]
#[user_data(user_data::ArcData<HelloWorld>)]
pub struct HelloWorld;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {

    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }

//    #[export]

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world. YE!");
    }

    #[export]
    fn _process(&self, owner: Node, delta: f64) {
        unsafe {
            if let Some(n) = owner.get_node(NodePath::from_str("Ship")) {
                let mut s : Spatial = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
//                s.rotate_y(delta);
            }
            if let Some(n) = owner.get_node(NodePath::from_str("CanvasLayer/Node2D")) {
//                if let Some(x) = n
                let mut s : Node2D = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
//                s.draw_rect(rect(10.0, 10.0, 200.0, 200.0), Color::rgb(1.0, 1.0, 0.0), true);
//                s.update();
//                s.rotate_y(delta);
            }
        }
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<GUIPaintNode>();

    let mut cmds = std::vec::Vec::new();
    cmds.push(DrawCmd::Rect { x: 0, y: 0, w: 100, h: 100, color: (255, 255, 0, 255) });
    cmds.push(DrawCmd::Rect { x: 50, y: 25, w: 100, h: 100, color: (0, 255, 0, 255) });

    let f =
        ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://fonts/main_font_normal.tres"),
            GodotString::from_str("DynamicFont"),
            false);
    let df : DynamicFont = f.and_then(|f| f.cast::<DynamicFont>()).unwrap();

    let mut d = SSCG.lock().unwrap();
    d.fonts = Some(Fonts {
        main_font: df,
    });
    std::mem::replace(&mut d.v, cmds);
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
