mod state;
mod system_map;
mod wl_gd_mod_resolver;
mod util;

#[macro_use]
extern crate lazy_static;
extern crate gdnative;
use gdnative::*;

use euclid::rect;
use euclid::vec2;
use sscg::tree_painter::{DrawCmd, TreePainter};
use sscg::gui::*;

//use sscg::gui;
//use sscg::logic;

use std::rc::Rc;
//use std::cell::RefCell;

use state::*;
use util::c2c;

#[derive(NativeClass)]
#[inherit(gdnative::Node2D)]
#[user_data(user_data::MutexData<GUIPaintNode>)]
pub struct GUIPaintNode {
    win: Window,
    cache: std::vec::Vec<Option<std::vec::Vec<DrawCmd>>>,
    w: i64,
    h: i64,
}

fn draw_cmds(xxo: i32, yyo: i32, cache: &mut std::vec::Vec<Option<std::vec::Vec<DrawCmd>>>,
             n: &mut Node2D,
             fh: &FontHolder,
             cmds: &[DrawCmd]) {
    for c in cmds {
        match c {
            DrawCmd::CacheDraw { w, h, id, cmds: cd_cmds } => {
                if *id >= cache.len() {
                    cache.resize(*id + 1, None)
                }
                cache[*id] = Some(cd_cmds.clone());
            },
            DrawCmd::DrawCache { x, y, w, h, id } => {
                let my_cmds = std::mem::replace(&mut cache[*id], None);
                draw_cmds(xxo + x, yyo + y, cache, n, fh, my_cmds.as_ref().unwrap());
                std::mem::replace(&mut cache[*id], my_cmds);
            },
            DrawCmd::Text { txt, align, color, x, y, w } => {
                unsafe {
                    let size =
                        fh.main_font.get_string_size(GodotString::from(txt));
                    let xo =
                        if *w as f32 > size.x  {
                            match *align {
                                1  => 0.0,
                                0  => (*w as f32 - size.x) / 2.0,
                                -1 => (*w as f32 - size.x),
                                _  => 0.0,
                            }
                        } else {
                            0.0
                        };
                    n.draw_string(
                        Some(fh.main_font.to_font()),
                        vec2(xxo as f32 + xo + *x as f32, yyo as f32 + *y as f32 + fh.main_font.get_ascent() as f32),
                        GodotString::from_str(txt),
                        c2c(*color),
                        *w as i64);
                }
            },
            DrawCmd::Circle { x, y, r, color } => {
                unsafe {
                    n.draw_circle(
                        vec2((xxo + *x) as f32,
                             (yyo + *y) as f32),
                        *r as f64,
                        c2c(*color));
                }
            },
            DrawCmd::FilledCircle { x, y, r, color } => {
                unsafe {
                    n.draw_circle(
                        vec2((xxo + *x) as f32,
                             (yyo + *y) as f32),
                        *r as f64,
                        c2c(*color));
                }
            },
            DrawCmd::Line { x, y, x2, y2, t, color } => {
                unsafe {
                    n.draw_line(
                        vec2((xxo + *x) as f32,
                             (yyo + *y) as f32),
                        vec2((xxo + *x2) as f32,
                             (yyo + *y2) as f32),
                        c2c(*color),
                        *t as f64,
                        true);
                }
            },
            DrawCmd::Rect { x, y, w, h, color } => {
                unsafe {
                    n.draw_rect(
                        rect((xxo + *x) as f32,
                             (yyo + *y) as f32,
                             *w as f32,
                             *h as f32),
                        c2c(*color),
                        false);
                }
            },
            DrawCmd::FilledRect { x, y, w, h, color } => {
                unsafe {
                    n.draw_rect(
                        rect((xxo + *x) as f32,
                             (yyo + *y) as f32,
                             *w as f32,
                             *h as f32),
                        c2c(*color),
                        true);
                }
            },
            _ => (),
        }
    }
}

#[methods]
impl GUIPaintNode {
    fn _init(_owner: Node2D) -> Self { Self { win: Window::new(), w: 0, h: 0, cache: vec![] } }

    #[export]
    fn _ready(&mut self, _owner: Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("NODE PAINT READY");
        self.win.w = 250;
        self.win.h = 250;
        self.win.x = 500;
        self.win.y = 750;
        self.win.title = String::from("HUD");
        let c1 = self.win.add_label(sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 }, sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255)));
        let c2 = self.win.add_label(
            sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 },
            sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255)).clickable());
        let c3 = self.win.add_label(
            sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 },
            sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255)).editable("."));
        self.win.child =
            self.win.add_layout(
                sscg::gui::Size { min_w: 10, w: 500, min_h: 10, h: 1000, margin: 0 },
                BoxDir::Vert(1), &vec![c1, c2, c3]);
    }

    #[export]
    fn on_resize(&mut self, mut s: Node2D, w: f64, h: f64) {
        self.w = w as i64;
        self.h = h as i64;
        dbg!("RESIZE {} {}", w, h);
        self.win.does_need_redraw();
        unsafe { s.update(); }
    }

    #[export]
    fn on_mouse_click(&mut self, mut s: Node2D, x: f64, y: f64) {
        self.win.handle_event(WindowEvent::Click(x as i32, y as i32));
        if let Some(s) = self.win.collect_activated_child() {
            dbg!("FOO", s);
        }
        if self.win.needs_redraw() { unsafe { s.update(); } }
    }

    #[export]
    fn on_mouse_move(&mut self, mut s: Node2D, x: f64, y: f64) {
        self.win.handle_event(WindowEvent::MousePos(x as i32, y as i32));
        if self.win.needs_redraw() { unsafe { s.update(); } }
    }

    #[export]
    fn on_input(&mut self, mut s: Node2D, character: i64) {
        if character > 0 {
            let c = std::char::from_u32(character as u32).unwrap_or('\0');
            let mut charstr = String::new();
            charstr.push(c);
            self.win.handle_event(WindowEvent::TextInput(charstr));

        } else if character < 0 {
            self.win.handle_event(WindowEvent::Backspace);
        }
        if self.win.needs_redraw() { unsafe { s.update(); } }
    }

    #[export]
    fn _draw(&mut self, mut s: Node2D) {
        let mut d = SSCG.lock().unwrap();
        let d2 = d.as_mut().unwrap();

        d2.tp.clear_cmds();
        self.win.draw(0, self.w as u32, self.h as u32, &mut d2.tp);
        let fh_rc = d2.fonts.clone();
        draw_cmds(0, 0, &mut self.cache, &mut s, &*fh_rc, d2.tp.ref_cmds());
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
    fn _process(&self, _owner: Node, _delta: f64) {
//        unsafe {
//            if let Some(n) = owner.get_node(NodePath::from_str("Ship")) {
//                let s : Spatial = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
//                s.rotate_y(delta);
//            }
//            if let Some(n) = owner.get_node(NodePath::from_str("CanvasLayer/Node2D")) {
//                if let Some(x) = n
//                let s : Node2D = n.cast().unwrap();
//                godot_print!("DELTA: {} : {}", s.get_name().to_string(), delta);
//                s.draw_rect(rect(10.0, 10.0, 200.0, 200.0), Color::rgb(1.0, 1.0, 0.0), true);
//                s.update();
//                s.rotate_y(delta);
//            }
//        }
    }
}

fn terminate(options: *mut gdnative::sys::godot_gdnative_terminate_options) {
    dbg!("*** terminate sscg native");
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    dbg!("*** init sscg native");
    handle.add_class::<HelloWorld>();
    handle.add_class::<GUIPaintNode>();
    handle.add_class::<system_map::SystemMap>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!(terminate);


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
