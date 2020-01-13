#[macro_use]
mod state;
mod system_map;
mod wl_gd_mod_resolver;
mod util;
mod voxel_structure;
mod voxeltree;
mod voxeltree_wlambda;
mod gd_voxel_impl;

#[macro_use]
extern crate lazy_static;
extern crate gdnative;

use gdnative::*;
use euclid::rect;
use euclid::vec2;
use euclid::Vector2D;
use sscg::tree_painter::{DrawCmd, FontSize};
use sscg::gui::*;
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
    textures: Option<std::vec::Vec<Texture>>,
    num_input: Option<(usize, f64, f64)>,
}

unsafe impl Send for GUIPaintNode { }

fn draw_cmds(xxo: i32, yyo: i32,
             cache: &mut std::vec::Vec<Option<std::vec::Vec<DrawCmd>>>,
             n: &mut Node2D,
             fh: &FontHolder,
             textures: &[Texture],
             cmds: &[DrawCmd])
{
    for c in cmds {
        match c {
            DrawCmd::CacheDraw { w: _w, h: _h, id, cmds: cd_cmds } => {
                if *id >= cache.len() {
                    cache.resize(*id + 1, None)
                }
                cache[*id] = Some(cd_cmds.clone());
            },
            DrawCmd::DrawCache { x, y, w: _w, h: _h, id } => {
                let my_cmds = std::mem::replace(&mut cache[*id], None);
                draw_cmds(xxo + x, yyo + y, cache, n, fh, textures, my_cmds.as_ref().unwrap());
                std::mem::replace(&mut cache[*id], my_cmds);
            },
            DrawCmd::Texture { txt_idx, x, y, w, h, centered } => {
                unsafe {
                    let txt = &textures[*txt_idx];
                    let sz  = txt.get_size();

                    let xo = if *centered { -(sz.x / 2.0) } else { 0.0 };
                    let yo = if *centered { -(sz.y / 2.0) } else { 0.0 };

                    let w = if *w == 0 { sz.x } else { *w as f32 };
                    let h = if *h == 0 { sz.y } else { *h as f32 };

                    let aspect = if sz.y > 0.0 { sz.x / sz.y } else { 1.0 };
                    let min_edge = w.min(h);
                    let (w, h) =
                        if sz.x > sz.y {
                            ((sz.x / sz.y) * min_edge, min_edge)
                        } else {
                            (min_edge, (sz.y / sz.x) * min_edge)
                        };

                    n.draw_texture_rect(
                        Some(txt.clone()),
                        rect(xo + (xxo + *x) as f32,
                             yo + (yyo + *y) as f32,
                             w,
                             h),
                        false,
                        Color::rgba(1.0, 1.0, 1.0, 1.0),
                        false,
                        None);
                }
            },
            DrawCmd::Text { txt, align, color, x, y, w, fs } => {
                unsafe {
                    let font : &DynamicFont =
                        match fs {
                            FontSize::Normal => &fh.main_font,
                            FontSize::Small  => &fh.small_font,
                        };
                    let size = font.get_string_size(GodotString::from(txt));

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
                        Some(font.to_font()),
                        vec2(xxo as f32 + xo + *x as f32,
                             yyo as f32 + *y as f32
                             + font.get_ascent() as f32),
                        GodotString::from_str(txt),
                        c2c(*color),
                        *w as i64);
                }
            },
            DrawCmd::Circle { x, y, r, color } => {
                unsafe {
                    println!("CIRCLE: {},{},{} : {:?}", x, y, r, color);
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
        Self {
            w:          0,
            h:          0,
            cache:      vec![],
            textures:   None,
            num_input:  None,
        }
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
    fn on_mouse_release(&mut self, mut s: Node2D, x: f64, y: f64) {
        lock_sscg!(sscg);

        if let Some((win_id, _, _)) = self.num_input {
            sscg.wm.borrow_mut()
                .windows[win_id]
                .as_mut().unwrap().handle_event(WindowEvent::Enter);
        }
        self.num_input = None;
    }

    #[export]
    fn on_mouse_click(&mut self, mut s: Node2D, x: f64, y: f64) {
        lock_sscg!(sscg);

        let mut numeric_input_start  = false;
        let mut numeric_input_win_id = 0;

        sscg.wm.borrow_mut().for_each_window_stop_on_true(
            |win| {
                let handled = win.handle_event(WindowEvent::Click(x as i32, y as i32));
                if handled && win.is_numeric_input_active() {
                    numeric_input_start  = true;
                    numeric_input_win_id = win.id;
                }
                handled
            });

        if numeric_input_start {
            self.num_input = Some((numeric_input_win_id, x, y));
        } else {
            self.num_input = None;
        }

        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn on_mouse_move(&mut self, mut s: Node2D, x: f64, y: f64,
                     button1_pressed: bool,
                     button2_pressed: bool,
                     mod1: bool,
                     mod2: bool) {
        lock_sscg!(sscg);

        if button2_pressed && self.num_input.is_some() {
            let (win_id, _x, _y) = self.num_input.unwrap();
            sscg.wm.borrow_mut().windows[win_id].as_mut().unwrap().handle_event(
                WindowEvent::NumericDrag {
                    dist: 0.0,
                    res: NumericDragRes::Original
                });

        } else if button1_pressed && self.num_input.is_some() {
            let (win_id, x1, y1) = self.num_input.unwrap();
            let orig_vec = vec2::<f64, f64>(x1, y1);
            let cur_vec  = vec2::<f64, f64>(x, y);
            sscg.wm.borrow_mut().windows[win_id].as_mut().unwrap().handle_event(
                WindowEvent::NumericDrag {
                    dist: ((orig_vec - cur_vec).length()
                           * (orig_vec - cur_vec).normalize().dot(vec2(1.0, 0.0)))
                          .round(),
                    res: if mod1 && mod2 { NumericDragRes::VeryFine }
                         else if mod1    { NumericDragRes::Fine }
                         else if mod2    { NumericDragRes::Coarse }
                         else            { NumericDragRes::Normal },
                });

        } else {
            self.num_input = None;
        }

        sscg.wm.borrow_mut().for_each_window_stop_on_true(
            |win| { win.handle_event(WindowEvent::MousePos(x as i32, y as i32)) });
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
            sscg.wm.borrow_mut().for_each_window_stop_on_true(
                move |win| { win.handle_event(WindowEvent::TextInput(charstr.clone())) });

        } else if character < 0 {
            let event = match character {
                -1 => WindowEvent::Backspace,
                -2 => WindowEvent::Enter,
                -3 => WindowEvent::Escape,
                _  => WindowEvent::Escape,
            };
            sscg.wm.borrow_mut().for_each_window_stop_on_true(
                |win| { win.handle_event(event.clone()) });
        }
        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn _process(&mut self, mut s: Node2D, _delta: f64) {
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
        }

        if sscg.wm.borrow_mut().some_win_needs_redraw(){
            unsafe { s.update(); }
        }
    }

    #[export]
    fn _draw(&mut self, mut s: Node2D) {
        lock_sscg!(sscg);

        if self.textures.is_none() {
            let ret = sscg.call_cb("on_texture_description", &vec![]);
            let mut textures : std::vec::Vec<Texture> = vec![];
            for t in ret.iter() {
                let txt = match &t.v_s_raw(0)[..] {
                    "image" => {
                        ResourceLoader::godot_singleton().load(
                            GodotString::from_str(t.v_s_raw(1)),
                            GodotString::from_str("ImageTexture"),
                            false).expect(&format!("Loading texture {}", t.s()))
                                  .cast::<Texture>()
                                  .expect(
                                      &format!("Failed casting to Texture {}",
                                               t.s()))
                    },
                    _ => { panic!(format!("Unknown texture type: {}", t.s())); },
                };

                textures.push(txt);
            }
            self.textures = Some(textures);
        }

        //d// println!("DRAW CALLBACK!");
        let tp = &mut sscg.tp;
        tp.clear_cmds();
        sscg.wm.borrow_mut().for_each_window_reverse(
            |win| win.draw(win.id, self.w as u32, self.h as u32, tp));
        let fh_rc = sscg.fonts.clone();
        //d// println!("DRAW CMDS {:?}", tp.ref_cmds());
        draw_cmds(0, 0, &mut self.cache, &mut s, &*fh_rc,
                  &self.textures.as_ref().expect("Textures loaded"),
                  tp.ref_cmds());
        sscg.wm.borrow_mut().redraw_done();
    }
}

fn terminate(_options: *mut gdnative::sys::godot_gdnative_terminate_options) {
    dbg!("*** terminate sscg native");
}

static mut OLDHOOK
    : Option<Box<dyn Fn(&std::panic::PanicInfo) + Sync + Send + 'static>> = None;

fn init_panic_hook() {
    unsafe {
        OLDHOOK = Some(std::panic::take_hook());
    }
    std::panic::set_hook(Box::new(|panic_info| {
        let mut loc_string = String::from("unknown location");
        if let Some(location) = panic_info.location() {
            loc_string =
                format!("file '{}' at line {}",
                        location.file(),
                        location.line());
        }

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            godot_print!("{}: panic occurred: {:?}", loc_string, s);
        } else {
            godot_print!("{}: unknown panic occurred", loc_string);
        }

        unsafe { (*(OLDHOOK.as_ref().unwrap()))(panic_info); }
    }));
}

fn init(handle: gdnative::init::InitHandle) {
    dbg!("*** init sscg native");
    init_panic_hook();
    handle.add_class::<GUIPaintNode>();
    handle.add_class::<system_map::SystemMap>();
    handle.add_class::<voxel_structure::VoxStruct>();

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
