mod state;
mod system_map;
mod wl_gd_mod_resolver;

#[macro_use]
extern crate lazy_static;
extern crate gdnative;
use gdnative::*;

use euclid::rect;
use euclid::vec2;
use sscg::tree_painter::{DrawCmd, TreePainter};

//use sscg::gui;
//use sscg::logic;

use std::rc::Rc;
//use std::cell::RefCell;

use state::*;

fn c2c(c: (u8, u8, u8, u8)) -> Color {
    Color::rgba(
        c.0 as f32 / 255.0,
        c.1 as f32 / 255.0,
        c.2 as f32 / 255.0,
        c.3 as f32 / 255.0)
}

#[derive(NativeClass)]
#[inherit(gdnative::Node2D)]
#[user_data(user_data::ArcData<GUIPaintNode>)]
pub struct GUIPaintNode { }

fn draw_cmds(n: &mut Node2D, fh: &FontHolder, cmds: &std::vec::Vec<DrawCmd>) {
    for c in cmds {
        match c {
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
                        vec2(xo + *x as f32, *y as f32),
                        GodotString::from_str(txt),
                        c2c(*color),
                        *w as i64);
                }
            },
            DrawCmd::Circle { x, y, r, color } => {
                unsafe {
                    n.draw_circle(
                        vec2(*x as f32,
                             *y as f32),
                        *r as f64,
                        c2c(*color));
                }
            },
            DrawCmd::FilledCircle { x, y, r, color } => {
                unsafe {
                    n.draw_circle(
                        vec2(*x as f32,
                             *y as f32),
                        *r as f64,
                        c2c(*color));
                }
            },
            DrawCmd::Line { x, y, x2, y2, t, color } => {
                unsafe {
                    n.draw_line(
                        vec2(*x as f32,
                             *y as f32),
                        vec2(*x2 as f32,
                             *y2 as f32),
                        c2c(*color),
                        *t as f64,
                        true);
                }
            },
            DrawCmd::Rect { x, y, w, h, color } => {
                unsafe {
                    n.draw_rect(
                        rect(*x as f32,
                             *y as f32,
                             *w as f32,
                             *h as f32),
                        c2c(*color),
                        false);
                }
            },
            DrawCmd::FilledRect { x, y, w, h, color } => {
                unsafe {
                    n.draw_rect(
                        rect(*x as f32,
                             *y as f32,
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
    fn _init(_owner: Node2D) -> Self { Self { } }

    #[export]
    fn _ready(&self, _owner: Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("NODE PAINT READY");
    }

    #[export]
    fn _draw(&self, mut s: Node2D) {
        let mut d = SSCG.lock().unwrap();
        let d2 = d.as_mut().unwrap();
//        if !d2.v.is_empty() {
//            self.cmds = std::mem::replace(&mut d2.v, std::vec::Vec::new());
//            godot_print!("GOT IT");
//        }

//        for v in self.cmds.iter() {
//            godot_print!("FO {:?}", v);
//        }

        let fh_rc = d2.fonts.clone();
        if !d2.v.is_empty() {
            draw_cmds(&mut s, &*fh_rc, &d2.v);
            d2.v.clear();
        }

        unsafe {
            s.draw_string(
                Some(fh_rc.main_font.to_font()),
                vec2(50.0, 50.0),
                GodotString::from_str("FÖRSTER"),
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
