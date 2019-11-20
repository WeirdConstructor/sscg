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
use std::rc::Rc;
use state::*;
use util::c2c;
use wlambda::VVal;

#[derive(NativeClass)]
#[inherit(gdnative::Node2D)]
#[user_data(user_data::MutexData<GUIPaintNode>)]
pub struct GUIPaintNode {
    cache: std::vec::Vec<Option<std::vec::Vec<DrawCmd>>>,
    w: i64,
    h: i64,
}

fn draw_cmds(xxo: i32, yyo: i32,
             cache: &mut std::vec::Vec<Option<std::vec::Vec<DrawCmd>>>,
             n: &mut Node2D,
             fh: &FontHolder,
             cmds: &[DrawCmd])
{
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
                        vec2(xxo as f32 + xo + *x as f32,
                             yyo as f32 + *y as f32
                             + fh.main_font.get_ascent() as f32),
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
    fn _init(_owner: Node2D) -> Self {
        Self { w: 0, h: 0, cache: vec![] }
    }

    #[export]
    fn _ready(&mut self, _owner: Node2D) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("NODE PAINT READY");

//        self.win.w = 250;
//        self.win.h = 250;
//        self.win.x = 500;
//        self.win.y = 750;
//        self.win.title = String::from("HUD");
//        let c1 = self.win.add_label(
//            sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 },
//            sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255)));
//        let c2 = self.win.add_label(
//            sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 },
//            sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255))
//            .clickable());
//        let c3 = self.win.add_label(
//            sscg::gui::Size { min_w: 10, w: 1000, min_h: 0, h: 0, margin: 0 },
//            sscg::gui::Label::new("Test123", (255, 0, 255, 255), (0, 0, 0, 255))
//            .editable("."));
//        self.win.child =
//            self.win.add_layout(
//                sscg::gui::Size { min_w: 10, w: 500, min_h: 10, h: 1000, margin: 0 },
//                BoxDir::Vert(1), &vec![c1, c2, c3]);
    }

    #[export]
    fn on_resize(&mut self, mut s: Node2D, w: f64, h: f64) {
        lock_sscg!(sscg);

        self.w = w as i64;
        self.h = h as i64;
        dbg!("RESIZE {} {}", w, h);
        sscg.wm.borrow_mut().for_each_window(|win| win.does_need_redraw());
        unsafe { s.update(); }
    }

    #[export]
    fn on_mouse_click(&mut self, mut s: Node2D, x: f64, y: f64) {
        lock_sscg!(sscg);

        sscg.wm.borrow_mut().for_each_window(
            |win| { win.handle_event(WindowEvent::Click(x as i32, y as i32)); });
        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn on_mouse_move(&mut self, mut s: Node2D, x: f64, y: f64) {
        lock_sscg!(sscg);

        sscg.wm.borrow_mut().for_each_window(
            |win| { win.handle_event(WindowEvent::MousePos(x as i32, y as i32)); });
        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn on_input(&mut self, mut s: Node2D, character: i64) {
        lock_sscg!(sscg);

        if character > 0 {
            let c = std::char::from_u32(character as u32).unwrap_or('\0');
            let mut charstr = String::new();
            charstr.push(c);
            sscg.wm.borrow_mut().for_each_window(
                move |win| { win.handle_event(WindowEvent::TextInput(charstr.clone())); });

        } else if character < 0 {
            sscg.wm.borrow_mut().for_each_window(
                |win| { win.handle_event(WindowEvent::Backspace); });
        }
        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn _process(&mut self, mut s: Node2D, delta: f64) {
        lock_sscg!(sscg);

        let acts = sscg.wm.borrow_mut().get_activated_childs();
        if let Some(acts) = acts {
            for (idx, lblref, cb) in acts {
                let args = vec![
                    VVal::Int(idx as i64),
                    VVal::new_str_mv(lblref)
                ];
                if let Err(e) = sscg.wlctx.call(&cb, &args) {
                    println!("ERROR IN WM CB: {}", e);
                }
            }
            if sscg.wm.borrow_mut().some_win_needs_redraw(){
                unsafe { s.update(); }
            }
        }
    }

    #[export]
    fn _draw(&mut self, mut s: Node2D) {
        lock_sscg!(sscg);

        println!("DRAW CALLBACK!");
        let tp = &mut sscg.tp;
        tp.clear_cmds();
        sscg.wm.borrow_mut().for_each_window(
            |win| win.draw(0, self.w as u32, self.h as u32, tp));
        let fh_rc = sscg.fonts.clone();
        println!("DRAW CMDS {:?}", tp.ref_cmds());
        draw_cmds(0, 0, &mut self.cache, &mut s, &*fh_rc, tp.ref_cmds());
    }
}


/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(gdnative::Node)]
#[user_data(user_data::ArcData<HelloWorld>)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {

    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: Node) {
        godot_print!("hello, world. YE!");
    }

    #[export]
    fn _process(&self, _owner: Node, _delta: f64) {
    }
}

fn terminate(options: *mut gdnative::sys::godot_gdnative_terminate_options) {
    dbg!("*** terminate sscg native");
}

fn init(handle: gdnative::init::InitHandle) {
    dbg!("*** init sscg native");
    handle.add_class::<HelloWorld>();
    handle.add_class::<GUIPaintNode>();
    handle.add_class::<system_map::SystemMap>();
}

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
