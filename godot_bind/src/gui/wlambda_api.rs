use std::rc::Rc;
use std::cell::RefCell;
use super::widgets::*;
use wlambda::{VVal, StackAction, VValUserData, Env};
//#[macro_use]
use wlambda::set_vval_method;

pub struct WindowManager {
    pub windows: std::vec::Vec<Option<Window>>,
    pub ev_cbs: std::vec::Vec<VVal>,
    pub need_redraw: bool,
}

pub fn window_manager_wlambda_obj(
    wm: std::rc::Rc<std::cell::RefCell<WindowManager>>)
    -> VVal
{
    let o = VVal::map();

    set_vval_method!(o, wm, set_window, Some(1), Some(3), env, argc, {
        let idx = env.arg(0).i() as usize;
        if argc > 1 {
            let win = vval2win(env.arg(1));
            let cb  = env.arg(2);
            wm.borrow_mut().set(idx, win, cb);
        } else {
            wm.borrow_mut().delete(idx);
        }

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, wm, set_label, Some(3), Some(3), env, _argc, {
        let idx    = env.arg(0).i() as usize;
        let lblref = env.arg(1).s_raw();
        let txt    = env.arg(2).s_raw();

        wm.borrow_mut().set_label_text(idx, &lblref, txt);

        Ok(VVal::Bol(true))
    });

    set_vval_method!(o, wm, get_label, Some(2), Some(2), env, _argc, {
        let idx    = env.arg(0).i() as usize;
        let lblref = env.arg(1).s_raw();

        Ok(wm.borrow_mut().get_label_text(idx, &lblref))
    });

    set_vval_method!(o, wm, get_state, Some(1), Some(1), env, _argc, {
        let idx    = env.arg(0).i() as usize;
        Ok(wm.borrow_mut().get_window_state(idx))
    });

    o
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: std::vec::Vec::new(),
            ev_cbs: std::vec::Vec::new(),
            need_redraw: false,
        }
    }

    pub fn for_each_window_reverse<F>(&mut self, mut winfun: F)
        where F: FnMut(&mut Window) -> ()
    {
        for w in self.windows.iter_mut().rev() {
            if w.is_none() { continue; }
            winfun(&mut *w.as_mut().unwrap())
        }
    }

    pub fn for_each_window_stop_on_true<F>(&mut self, mut winfun: F)
        where F: FnMut(&mut Window) -> bool
    {
        for w in self.windows.iter_mut() {
            if w.is_none() { continue; }
            if winfun(&mut *w.as_mut().unwrap()) {
                break;
            }
        }
    }

    pub fn for_each_window<F>(&mut self, mut winfun: F)
        where F: FnMut(&mut Window) -> ()
    {
        for w in self.windows.iter_mut() {
            if w.is_none() { continue; }
            winfun(&mut *w.as_mut().unwrap())
        }
    }

    pub fn some_win_needs_redraw(&mut self) -> bool {
        let mut need_redraw = self.need_redraw;
        self.for_each_window(
            |win| if win.needs_redraw() { need_redraw = true });
        need_redraw
    }

    pub fn get_activated_childs(&mut self)
        -> Option<std::vec::Vec<(usize, String, VVal)>> {

        let mut activations : Option<std::vec::Vec<(usize, String, VVal)>> = None;

        for (idx, (w, cb)) in self.windows.iter_mut().zip(self.ev_cbs.iter()).enumerate() {
            if let Some(w) = w {
                if let Some(lblref) = w.collect_activated_child() {
                    if activations.is_none() { activations = Some(vec![]); }
                    if let Some(ref mut a) = activations {
                        a.push((idx, lblref.to_string(), cb.clone()));
                    }
                }
            }
        }

        activations
    }

    pub fn set_label_text(&mut self, idx: usize, lblref: &str, text: String) {
        if idx >= self.windows.len() {
            return;
        }
        if self.windows[idx].is_none() {
            return;
        }

        if let Some(e) = self.windows.get_mut(idx) {
            e.as_mut().unwrap().set_label_text(lblref, text);
        }
    }

    pub fn get_window_state(&self, idx: usize) -> VVal {
        if idx >= self.windows.len() {
            return VVal::None;
        }
        if self.windows[idx].is_none() {
            return VVal::None;
        }

        if let Some(e) = self.windows.get(idx) {
            let txts = e.as_ref().unwrap().collect_label_texts();
            let s = VVal::map();
            let v = VVal::map();
            for (lblref, text) in txts.into_iter() {
                v.set_key_str(&lblref, VVal::new_str_mv(text));
            }
            s.set_key_str("labels", v);
            s
        } else {
            VVal::None
        }
    }

    pub fn get_label_text(&self, idx: usize, lblref: &str) -> VVal {
        if idx >= self.windows.len() {
            return VVal::None;
        }
        if self.windows[idx].is_none() {
            return VVal::None;
        }

        if let Some(e) = self.windows.get(idx) {
            let r = e.as_ref().unwrap().get_label_text(lblref);
            if r.is_some() {
                VVal::new_str_mv(r.unwrap())
            } else {
                VVal::None
            }
        } else {
            VVal::None
        }
    }

    pub fn new_ref() -> std::rc::Rc<std::cell::RefCell<Self>> {
        std::rc::Rc::new(std::cell::RefCell::new(Self::new()))
    }

    pub fn redraw_done(&mut self) {
        self.need_redraw = false;
    }

    pub fn delete(&mut self, idx: usize) {
        if idx >= self.windows.len() { return; }
        self.windows[idx] = None;
        self.ev_cbs[idx] = VVal::None;
        self.need_redraw = true;
    }

    pub fn set(&mut self, idx: usize, mut win: Window, cb: VVal) -> usize {
        if idx >= self.windows.len() {
            self.windows.resize(idx + 1, None);
            self.ev_cbs.resize(idx + 1, VVal::None);
        }
        win.id = idx;
        self.windows[idx] = Some(win);
        self.ev_cbs[idx]  = cb;
        idx
    }
}

#[derive(Clone)]
pub struct WindowManagerWlWrapper(Rc<RefCell<WindowManager>>);
impl WindowManagerWlWrapper {
    pub fn vval_from(r: Rc<RefCell<WindowManager>>) -> VVal {
        VVal::Usr(Box::new(WindowManagerWlWrapper(r)))
    }
}

fn shiftaddup4(u: u8) -> u8 { (u << 4) | u }

pub fn color_hex24tpl(s: &str) -> (u8, u8, u8, u8) {
    match s.len() {
        8 => (
            u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
            u8::from_str_radix(&s[6..8], 16).unwrap_or(255)
        ),
        6 => (
            u8::from_str_radix(&s[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&s[2..4], 16).unwrap_or(0),
            u8::from_str_radix(&s[4..6], 16).unwrap_or(0),
            255
        ),
        4 => (
            shiftaddup4(u8::from_str_radix(&s[0..1], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[1..2], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[2..3], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[3..4], 16).unwrap_or(0xF)),
        ),
        3 => (
            shiftaddup4(u8::from_str_radix(&s[0..1], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[1..2], 16).unwrap_or(0)),
            shiftaddup4(u8::from_str_radix(&s[2..3], 16).unwrap_or(0)),
            255
        ),
        _ => (255, 0, 255, 255),
    }
}

fn vval2size(v: VVal) -> Size {
    let mut s = Size {
        min_w:    0,
        min_h:    0,
        w:     1000,
        h:     1000,
        margin:   0,
    };

    s.min_w  = v.get_key("min_w") .unwrap_or(VVal::Int(0)).i() as u32;
    s.min_h  = v.get_key("min_h") .unwrap_or(VVal::Int(0)).i() as u32;
    s.w      = v.get_key("w")     .unwrap_or(VVal::Int(0)).i() as u32;
    s.h      = v.get_key("h")     .unwrap_or(VVal::Int(0)).i() as u32;
    s.margin = v.get_key("margin").unwrap_or(VVal::Int(0)).i() as u32;

    s
}

fn vval2widget(v: VVal, win: &mut Window) -> usize {
    let mut childs = vec![];
    if let Some(VVal::Lst(l)) = v.get_key("childs") {
        for w in l.borrow().iter() {
            childs.push(vval2widget(w.clone(), win));
        }
    }

    match &v.get_key("t").unwrap_or(VVal::None).s_raw()[..] {
        "vbox" => {
            return win.add_layout(
                vval2size(v.clone()),
                BoxDir::Vert(v.v_ik("spacing") as u32),
                v.v_ik("border") as i32,
                color_hex24tpl(&v.v_s_rawk("border_color")),
                &childs);
        },
        "hbox" => {
            return win.add_layout(
                vval2size(v.clone()),
                BoxDir::Hori(v.v_ik("spacing") as u32),
                v.v_ik("border") as i32,
                color_hex24tpl(&v.v_s_rawk("border_color")),
                &childs);
        },
        "texture" => {
            return win.add_texture(
                vval2size(v.clone()),
                v.v_ik("idx") as usize);
        },
        "canvas" => {
            let mut cv = Canvas::new(v.v_s_rawk("ref"));
            for (elem, _) in v.clone().v_k("cmds").iter() {
                let id = if elem.v_(1).is_none() {
                    None
                } else {
                    Some(elem.v_i(1) as usize)
                };
                match &elem.v_s_raw(0)[..] {
                    "circle" => {
                        cv.push(CanvasCmd::Circle(
                            id,
                            elem.v_i(2) as i32,
                            elem.v_i(3) as i32,
                            elem.v_i(4) as u32,
                            color_hex24tpl(&elem.v_s_raw(5))));
                    },
                    "text" => {
                        cv.push(CanvasCmd::Text(
                            id,
                            elem.v_i(2) as i32,
                            elem.v_i(3) as i32,
                            elem.v_i(4) as u32, // w
                            elem.v_i(5) as i32, // align
                            elem.v_s_raw(6),
                            match elem.v_i(7) {
                                -1 => FontSize::Small,
                                _  => FontSize::Normal,
                            },
                            color_hex24tpl(&elem.v_s_raw(8))));
                    },
                    "rect" => {
                        cv.push(CanvasCmd::Rect(
                            id,
                            elem.v_i(2) as i32,
                            elem.v_i(3) as i32,
                            elem.v_i(4) as u32,
                            elem.v_i(5) as u32,
                            color_hex24tpl(&elem.v_s_raw(6))));
                    },
                    "rect_filled" => {
                        cv.push(CanvasCmd::RectFilled(
                            id,
                            elem.v_i(2) as i32,
                            elem.v_i(3) as i32,
                            elem.v_i(4) as u32,
                            elem.v_i(5) as u32,
                            color_hex24tpl(&elem.v_s_raw(6))));
                    },
                    "line" => {
                        cv.push(CanvasCmd::Line(
                            elem.v_i(2) as i32,
                            elem.v_i(3) as i32,
                            elem.v_i(4) as i32,
                            elem.v_i(5) as i32,
                            elem.v_i(6) as u32,
                            color_hex24tpl(&elem.v_s_raw(7))));
                    },
                    _ => {},
                }
            }
            return win.add_canvas(vval2size(v.clone()), cv);
        },
        _ => ()
    }

    let mut lbl =
        Label::new(
            &v.v_s_rawk("text"),
            color_hex24tpl(&v.v_s_rawk("fg")),
            color_hex24tpl(&v.v_s_rawk("bg")))
        .lblref(&v.v_s_rawk("ref"));

    if &v.v_s_rawk("font")[..] == "small" {
        lbl = lbl.small_font();
    }

    let lbl = match &v.v_s_rawk("t")[..] {
        "l_button" => lbl.left().clickable(),
        "r_button" => lbl.right().clickable(),
        "c_button" => lbl.center().clickable(),
        "field" => {
            lbl.left().editable(
                &v.get_key("regex").unwrap_or(VVal::new_str(".*")).s_raw())
        },
        "field_numeric" => {
            lbl.left().editable(".*").numeric(NumericInput {
                default:        v.v_fk("num_default"),
                normal_step:    v.v_fk("num_normal_step"),
                fine_step:      v.v_fk("num_fine_step"),
                very_fine_step: v.v_fk("num_very_fine_step"),
                coarse_step:    v.v_fk("num_coarse_step"),
            })
        },
        "c_text"  => lbl.center().wrap(),
        "l_text"  => lbl.left().wrap(),
        "r_text"  => lbl.right().wrap(),
        "c_label" => lbl.center(),
        "l_label" => lbl.left(),
        "r_label" => lbl.right(),
        _ => lbl,
    };

    win.add_label(vval2size(v.clone()), lbl)
}

fn vval2win(v: VVal) -> Window {
    let mut w = Window::new();
    w.x     = v.get_key("x").unwrap_or(VVal::Int(0)).i() as i32;
    w.y     = v.get_key("y").unwrap_or(VVal::Int(0)).i() as i32;
    w.w     = v.get_key("w").unwrap_or(VVal::Int(500)).i() as i32;
    w.h     = v.get_key("h").unwrap_or(VVal::Int(500)).i() as i32;
    w.title = v.get_key("title").unwrap_or(VVal::new_str("Unnamed")).s_raw();
    if let Some(tc) = v.get_key("title_color") {
        w.title_color = color_hex24tpl(&tc.s_raw());
    }
    let id = vval2widget(v.get_key("child").unwrap_or(VVal::None), &mut w);
    w.child = id;

    w
}

impl VValUserData for WindowManagerWlWrapper {
    fn s(&self) -> String { format!("$<WindowManager>") }
    fn i(&self) -> i64 { 0 }
    fn call(&self, env: &mut Env) -> Result<VVal, StackAction> {
        let args = env.argv_ref();
        if args.len() < 1 {
            return Err(StackAction::panic_msg(
                format!("{} called with too few arguments", self.s())));
        }
        match &args[0].s_raw()[..] {
            "set_window" => {
                if args.len() < 4 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_window` called with too few arguments",
                                self.s())));
                }


                let idx = args[1].i();
                if !args[2].is_none() {
                    let win = vval2win(args[2].clone());
                    let cb  = args[3].clone();

                    self.0.borrow_mut().set(idx as usize, win, cb);
                } else {
                    self.0.borrow_mut().delete(idx as usize);
                }

                Ok(VVal::Bol(true))
            },
            "set_label" => {
                if args.len() < 3 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :set_label` called with too few arguments",
                                self.s())));
                }

                let idx    = args[1].i();
                let lblref = args[2].s_raw();
                let txt    = args[3].s_raw();
                self.0.borrow_mut().set_label_text(idx as usize, &lblref, txt);

                Ok(VVal::Bol(true))
            },
            "get_label" => {
                if args.len() < 2 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :get_label` called with too few arguments",
                                self.s())));
                }

                let idx    = args[1].i();
                let lblref = args[2].s_raw();
                Ok(self.0.borrow_mut().get_label_text(idx as usize, &lblref))
            },
            "get_state" => {
                if args.len() < 1 {
                    return Err(StackAction::panic_msg(
                        format!("`{} :get_label` called with too few arguments",
                                self.s())));
                }

                let idx = args[1].i();
                Ok(self.0.borrow_mut().get_window_state(idx as usize))
            },
            _ => Ok(VVal::None)
        }
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any { self }
    fn clone_ud(&self) -> Box<dyn wlambda::vval::VValUserData> {
        Box::new(self.clone())
    }
}

