use regex::Regex;
use crate::logic::GamePainter;
pub use crate::logic::FontSize;

#[derive(Debug, Clone)]
pub enum Widget {
    Layout(usize, Size, Layout),
    Label(usize, Size, Label),
    Canvas(usize, Size, Canvas),
}

fn calc_m_wo_spacing(child_count: usize, spacing: u32, border: i32, mw: u32) -> u32 {
    let mw = if mw < (4 * border) as u32 { 0 } else { mw - (4 * border) as u32 };
    let mut child_spacings =
        if child_count <= 1 { 0 }
        else { child_count - 1 };
    let mut mw_wo_spacing =
        mw as i32 - (child_spacings as u32 * spacing) as i32;

    if mw_wo_spacing < 0 { 0 }
    else { mw_wo_spacing as u32 }
}

impl Widget {
    pub fn id(&self) -> usize {
        match self {
            Widget::Layout(id, _, _) => *id,
            Widget::Label(id, _, _)  => *id,
            Widget::Canvas(id, _, _) => *id,
        }
    }
    pub fn calc_feedback<P>(&self, max_w: u32, max_h: u32, p: &mut P)
        -> WidgetFeedback where P: GamePainter {

        let pos = p.get_screen_pos(0, 0);
        let (id, (mw, mh)) = match self {
            Widget::Layout(id, l, _) => {
                p.push_add_offs(l.margin as i32, l.margin as i32);
                (*id, l.size(max_w, max_h))
            },
            Widget::Label(id, l, _)  => {
                p.push_add_offs(l.margin as i32, l.margin as i32);
                (*id, l.size(max_w, max_h))
            },
            Widget::Canvas(id, l, _)  => {
                p.push_add_offs(l.margin as i32, l.margin as i32);
                (*id, l.size(max_w, max_h))
            },
        };

        WidgetFeedback {
            id,
            x: pos.0 as u32,
            y: pos.1 as u32,
            w: mw,
            h: mh,
            sub: None,
        }
    }

    pub fn draw_label_editable<P>(
        &self,
        lbl: &Label,
        bg_color: (u8, u8, u8, u8),
        th: u32,
        mw: &mut u32, mh: &mut u32, fs: FontSize, p: &mut P)
        where P: GamePainter {

        let border_pad = 4;
        *mw = *mw + 2 * border_pad;
        p.draw_line(1, 0, 1, *mh as i32 - 1, 2, bg_color);
        p.draw_line(
            *mw as i32 - 1, 0,
            *mw as i32 - 1,
            *mh as i32 - 1, 2, bg_color);
        let text_field_width = *mw - 2 * border_pad;
        p.draw_rect_filled(
            border_pad as i32, 0,
            text_field_width, *mh, lbl.bg_color);
        p.draw_text(
            border_pad as i32, ((*mh - th) / 2) as i32,
            text_field_width, lbl.fg_color,
            None, lbl.align, &lbl.text, fs);
    }

    pub fn draw_label_multiline<P>(
        &self,
        lbl: &Label,
        bg_color: (u8, u8, u8, u8),
        _tw: u32, _th: u32,
        mw: &mut u32, mh: &mut u32, fs: FontSize, p: &mut P)
        where P: GamePainter {

        let txt = &lbl.text;
        let mut line = String::from("");
        let mut y : i32 = 0;

        let mut lines : std::vec::Vec<(i32, i32, String)> = vec![];
        for c in txt.chars() {
            line.push(c);
            let (tw, th) = p.text_size(&line, fs);
            if (y as u32 + th) > *mh { line = String::from(""); break; }
            if tw > *mw {
                if line.len() > 1 { line.pop(); }
                if line.len() > 1 {
                    lines.push((0, y, line));
                    line = String::from("");
                    line.push(c);
                } else {
                    lines.push((0, y, line));
                    line = String::from("");
                }
                y += th as i32;
            }
        }

        if line.len() > 0 {
            lines.push((0, y, line));
        }

        let text_lines_h = y as u32;

        let yo = if text_lines_h < *mh { (*mh - text_lines_h) / 2 }
                 else { 0 };
        p.draw_rect_filled(
            0, 0, *mw, *mh, bg_color);
        for (x, y, l) in lines.iter() {
            p.draw_text(
                *x, *y + yo as i32, *mw,
                lbl.fg_color, None, lbl.align, &l, fs);
        }
    }

    pub fn draw_label<P>(
        &self,
        lbl: &Label,
        bg_color: (u8, u8, u8, u8),
        _tw: u32, th: u32,
        mw: &mut u32, mh: &mut u32, fs: FontSize, p: &mut P)
        where P: GamePainter {

        if lbl.clickable {
            let mut corner_radius : u32 = if *mw < *mh { *mw / 4 } else { *mh / 4 };
            if corner_radius < (th / 2) { corner_radius = th / 2; }

            *mh = corner_radius * 2;
            if *mh < th { *mh = th; }
            if *mh % 2 == 0 { *mh += 1; }

            let text_pad = 4;

            let mut text_width : i32 = *mw as i32;
            let mut xo         : i32 = 0;
            match lbl.align {
                -1 => {
                    p.draw_dot(
                        corner_radius as i32,
                        corner_radius as i32,
                        corner_radius,
                        bg_color);
                    if text_pad > 0 {
                        p.draw_rect_filled(
                            (*mw - text_pad) as i32, 0,
                            text_pad, *mh, bg_color);
                    }
                    xo = corner_radius as i32;
                    text_width = *mw as i32 - (text_pad + corner_radius) as i32;
                },
                1 => {
                    p.draw_dot(
                        *mw as i32 - corner_radius as i32,
                        corner_radius as i32,
                        corner_radius,
                        bg_color);
                    p.draw_rect_filled(
                        0, 0, text_pad, *mh, bg_color);
                    xo = text_pad as i32;
                    text_width = *mw as i32 - (text_pad + corner_radius) as i32;
                },
                2 => { },
                _ => {
                    p.draw_dot(
                        corner_radius as i32,
                        corner_radius as i32,
                        corner_radius,
                        bg_color);
                    p.draw_dot(
                        *mw as i32 - corner_radius as i32,
                        corner_radius as i32,
                        corner_radius,
                        bg_color);
                    xo = corner_radius as i32;
                    text_width = *mw as i32 - 2 * corner_radius as i32;
                },
            }

            let text_width : u32 =
                if text_width < 0 { 0 } else { text_width as u32 };

            p.draw_rect_filled(
                xo, 0, text_width, *mh, bg_color);
            p.draw_text(
                xo, ((*mh - th) / 2) as i32, text_width as u32,
                lbl.fg_color, None, lbl.align, &lbl.text, fs);
        } else {
            let border_pad = 4;
            let text_field_width = *mw - 2 * border_pad;
            p.draw_rect_filled(
                0, 0, text_field_width + 2 * border_pad, *mh, lbl.bg_color);
            p.draw_text(
                border_pad as i32, ((*mh - th) / 2) as i32,
                text_field_width, lbl.fg_color,
                None, lbl.align, &lbl.text, fs);
        }

    }

    pub fn draw<P>(&self, win: &Window, fb: &mut [WidgetFeedback],
                   max_w: u32, max_h: u32, p: &mut P) -> (u32, u32)
        where P: GamePainter {

        let mut w_fb = self.calc_feedback(max_w, max_h, p);
        let (mut mw, mut mh) = (w_fb.w, w_fb.h);
        match self {
            Widget::Layout(_id, _size, c) => {
                let border = c.border;
                p.push_add_offs(border * 2, border * 2);

                match c.dir {
                    BoxDir::Vert(spacing) => {
                        let mut offs = 0;
                        let mh_wo_spacing =
                            calc_m_wo_spacing(c.childs.len(), spacing, border, mh);
                        let mw_wo_border =
                            if mw < (4 * border) as u32 { 0 }
                            else { mw - (4 * border) as u32 };

                        for c_id in c.childs.iter() {
                            if offs > 0 { offs += spacing as i32; }
                            p.push_add_offs(0, offs);
                            let (w, h) =
                                win.widgets[*c_id].draw(win, fb, mw_wo_border, mh_wo_spacing, p);
                            if w > mw { mw = w }
                            p.pop_offs();
                            offs += h as i32;
                        }
                        if offs as u32 > mh { mh = offs as u32; }
                    },
                    BoxDir::Hori(spacing) => {
                        let mut offs = 0;
                        let mw_wo_spacing =
                            calc_m_wo_spacing(c.childs.len(), spacing, border, mw);
                        let mh_wo_border =
                            if mh < (4 * border) as u32 { 0 }
                            else { mh - (4 * border) as u32 };

                        for c_id in c.childs.iter() {
                            if offs > 0 { offs += spacing as i32; }
                            p.push_add_offs(offs, 0);
                            let (w, h) =
                                win.widgets[*c_id].draw(win, fb, mw_wo_spacing, mh_wo_border, p);
                            if h > mh { mh = h }
                            p.pop_offs();
                            offs += w as i32;
                        }
                        if offs as u32 > mw { mw = offs as u32; }
                    },
                };

                p.pop_offs();

                p.draw_rect_filled(0, 0, mw, border as u32, c.border_color);
                p.draw_rect_filled(0, mh as i32 - border, mw, border as u32, c.border_color);
                p.draw_rect_filled(0, 0, border as u32, mh, c.border_color);
                p.draw_rect_filled(mw as i32 - border, 0, border as u32, mh, c.border_color);
            },
            Widget::Label(id, _size, lbl) => {
                let mut bg_color =
                    if let Some(hchld_id) = win.hover_child {
                        if *id == hchld_id { lbl.hlt_color }
                        else { lbl.bg_color } }
                    else { lbl.bg_color };

                if let Some(fchld_id) = win.focus_child {
                    if *id == fchld_id { bg_color = lbl.hlt_color; }
                }

                let txt = lbl.text.clone();
                let (tw, th) = p.text_size(&txt, lbl.font_size);
                //d// println!("TEXTS {}|{:?} => {},{}", txt, lbl.font_size, tw, th);

                if mw == 0 { mw = max_w; }
                if mw == 0 { mw = th; }
                if mh == 0 { mh = th; }
                if mh < th { mh = th; }

                if lbl.editable {
                    self.draw_label_editable(
                        lbl, bg_color, th, &mut mw, &mut mh, lbl.font_size, p);

                } else if lbl.wrap && tw > mw {
                    self.draw_label_multiline(
                        lbl, bg_color, tw, th, &mut mw, &mut mh, lbl.font_size, p);

                } else {
                    self.draw_label(lbl, bg_color, tw, th, &mut mw, &mut mh, lbl.font_size, p);
                }
            },
            Widget::Canvas(id, _size, cv) => {
                let min = if mw < mh { mw } else { mh };

                let mut hovered = false;
                if let Some(hchld_id) = win.hover_child {
                    if *id == hchld_id { hovered = true; }
                }

                for cmd in cv.cmds.iter() {
                    match cmd {
                        CanvasCmd::Circle( sid, x, y, r, clr ) => {
                            let x = p2r(min, *x);
                            let y = p2r(min, *y);
                            let r = p2r(min, *r as i32);

                            let mut clr = *clr;

                            if let Some(sid) = sid {
                                let x = if x < r { 0 } else { x - r };
                                let y = if y < r { 0 } else { y - r };
                                w_fb.add_sub_fb(*sid, x, y, 2 * r, 2 * r);

                                if let Some(schld_id) = win.hover_sub_child {
                                    if schld_id == *sid {
                                        clr = (255 - clr.0, 255 - clr.1,
                                               255 - clr.2, clr.3);
                                    }
                                }
                            }

                            p.draw_circle(x as i32, y as i32, r, clr);
                        },
                        CanvasCmd::Text( sid, x, y, w, align, s, fs, clr) => {
                            let x = p2r(min, *x);
                            let y = p2r(min, *y);
                            let w = p2r(min, *w as i32);
                            p.draw_text(
                                x as i32, y as i32, w, *clr, None, *align, &s, *fs);
                        },
                        CanvasCmd::CircleFilled( sid, x, y, r, clr ) => {
                            let x = p2r(min, *x);
                            let y = p2r(min, *y);
                            let r = p2r(min, *r as i32);
                            p.draw_circle(x as i32, y as i32, r, *clr);
                        },
                        CanvasCmd::Line( x1, y1, x2, y2, t, clr ) => {
                            p.draw_line(
                                p2r(min, *x1) as i32,
                                p2r(min, *y1) as i32,
                                p2r(min, *x2) as i32,
                                p2r(min, *y2) as i32,
                                *t,
                                *clr);
                        },
                        CanvasCmd::Rect( sid, x1, y1, rw, rh, clr ) => {
                            p.draw_rect(
                                p2r(min, *x1) as i32,
                                p2r(min, *y1) as i32,
                                p2r(min, *rw as i32),
                                p2r(min, *rh as i32),
                                *clr);
                        },
                        CanvasCmd::RectFilled( sid, x1, y1, rw, rh, clr ) => {
                            p.draw_rect_filled(
                                p2r(min, *x1) as i32,
                                p2r(min, *y1) as i32,
                                p2r(min, *rw as i32),
                                p2r(min, *rh as i32),
                                *clr);
                        },
                    }
                }
            },
        }
        p.pop_offs();
        w_fb.w = mw;
        w_fb.h = mh;
        fb[self.id()] = w_fb;
        (mw, mh)
    }
}

#[derive(Debug, Clone)]
pub struct WidgetFeedback {
    id: usize,
    x:  u32,
    y:  u32,
    w:  u32,
    h:  u32,
    sub: Option<std::vec::Vec<WidgetFeedback>>,
}

fn set_fb_vec(v: &mut std::vec::Vec<WidgetFeedback>, fb: WidgetFeedback) {
    if fb.id >= v.len() {
        v.resize(fb.id + 1, WidgetFeedback::new());
    }
    let id = fb.id;
    v[id] = fb;
}

impl WidgetFeedback {
    pub fn new() -> Self {
        Self {
            id: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            sub: None,
        }
    }

    pub fn offs(&mut self, x: u32, y: u32) {
        self.x += x;
        self.y += y;
        if let Some(ref mut sv) = self.sub {
            for s in sv.iter_mut() {
                s.offs(x, y);
            }
        }
    }

    pub fn add_sub_fb(&mut self, id: usize, x: u32, y: u32, w: u32, h: u32) {
        if self.sub.is_none() {
            self.sub = Some(vec![]);
        }
        let fb = Self::from(id, self.x + x, self.y + y, w, h);
        if let Some(ref mut sv) = self.sub {
            set_fb_vec(sv, fb);
        }
    }

    pub fn from(id: usize, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { id, x, y, w, h, sub: None }
    }

    pub fn is_inside(&self, x: u32, y: u32) -> bool {
           x >= self.x && x <= (self.x + self.w)
        && y >= self.y && y <= (self.y + self.h)
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub title:    String,
    pub title_color: (u8, u8, u8, u8),
    pub id:       usize,
    widgets:      std::vec::Vec<Widget>,
    feedback:     std::vec::Vec<WidgetFeedback>,
    pub child:    usize,
    focus_child:  Option<usize>,
    hover_child:  Option<usize>,
    hover_sub_child: Option<usize>,
    activ_child:  Option<usize>,
    activ_sub_child: Option<usize>,
    pub x:        i32,
    pub y:        i32,
    pub w:        i32,
    pub h:        i32,
    needs_redraw: bool,
    win_feedback: WidgetFeedback,
}

/// All values are in 0.1% scale. that means, to represent 100% you have to
/// supply 1000 to ratio to get the full value.
fn p2r(value: u32, ratio: i32) -> u32 {
    if ratio < 0 { (-ratio) as u32 }
    else { (value * ratio as u32) / 1000 }
}

pub enum WindowEvent {
    MousePos(i32, i32),
    Click(i32, i32),
    TextInput(String),
    Backspace,
}

#[allow(dead_code)]
impl Window {
    pub fn new() -> Self {
        Self {
            id:                 0,
            title:              String::from(""),
            title_color:        (255, 128, 128, 255),
            widgets:            std::vec::Vec::new(),
            feedback:           std::vec::Vec::new(),
            child:              0,
            focus_child:        None,
            hover_child:        None,
            hover_sub_child:    None,
            activ_child:        None,
            activ_sub_child:    None,
            x:                  0,
            y:                  0,
            w:                  0,
            h:                  0,
            needs_redraw:       true,
            win_feedback:       WidgetFeedback::new(),
        }
    }

    pub fn draw<P>(&mut self, id: usize, max_w: u32, max_h: u32, p: &mut P)
        where P: GamePainter {

        let mut feedback = std::vec::Vec::new();
        feedback.resize(self.widgets.len(), WidgetFeedback::new());
        let child = &self.widgets[self.child];

        let mut w_fb = WidgetFeedback::new();

        w_fb.x = p2r(max_w, self.x);
        w_fb.y = p2r(max_h, self.y);
        if self.x >= 0 {
            w_fb.w = p2r(max_w, self.w);
        } else {
            if w_fb.x > max_w {
                w_fb.w = 10;
            } else {
                w_fb.w = p2r(max_w - w_fb.x, self.w);
            }
        }
        if self.y >= 0 {
            w_fb.h = p2r(max_h, self.h);
        } else {
            if w_fb.y > max_h {
                w_fb.h = 10;
            } else {
                w_fb.h = p2r(max_h - w_fb.y, self.h);
            }
        }

        let mut ts = p.text_size(&self.title, FontSize::Normal);
        let corner_radius   : u32 = ts.1 / 2;
        let text_lr_pad     : i32 = 4;
        let padding         : i32 = 4;
        let title_color = self.title_color;
        let min_text_width : u32 = 20;

        // calculate min window size
        let min_size_due_to_decor =
            ((4 * padding as u32) + (4 * corner_radius)) as u32
            + (2 * text_lr_pad) as u32
            + min_text_width;
        if w_fb.w <= min_size_due_to_decor {
            w_fb.w = min_size_due_to_decor;
        }
        if w_fb.h <= min_size_due_to_decor {
            w_fb.h = (2 * padding as u32) + (2 * corner_radius);
        }

        // adjust text width
        let available_text_width =
            w_fb.w - (min_size_due_to_decor - min_text_width);
        ts.0 =
            if available_text_width <= 0 { min_text_width }
            else if ts.0 > available_text_width { available_text_width }
            else { ts.0 };

        p.push_add_offs(w_fb.x as i32, w_fb.y as i32);
        if self.needs_redraw {
            //d// println!("REDRAW {}", self.needs_redraw);
        }
        p.declare_cache_draw(
            0, 0,
            w_fb.w,
            w_fb.h,
            id, self.needs_redraw);
        if self.needs_redraw {
            self.needs_redraw = false;

            // window background rect
            p.draw_rect_filled(
                0, padding,
                w_fb.w - padding as u32 - corner_radius,
                w_fb.h - padding as u32,
                (0, 0, 0, 255));
            p.push_add_offs(padding, padding);

            // left round circle
            p.draw_dot(
                corner_radius as i32,
                corner_radius as i32,
                corner_radius,
                title_color);
            // extension of left circle to text
            p.draw_rect_filled(
                corner_radius as i32, 0,
                2 * corner_radius + 1,
                2 * corner_radius + 1,
                title_color);
            // title text
            let text_pos = 3 * corner_radius as i32 + text_lr_pad;
            p.draw_text(
                text_pos, 0, ts.0,
                title_color, None, 1, &self.title, FontSize::Normal);
            let after_text = text_pos + text_lr_pad + ts.0 as i32;
            let after_text_to_win_max_x = w_fb.w as i32 - after_text;
            // rectangle from text end to right circle
            p.draw_rect_filled(
                after_text, 0,
                (after_text_to_win_max_x - 2 * corner_radius as i32) as u32,
                2 * corner_radius + 1,
                title_color);
            // right circle
            let right_dot_x =
                after_text + after_text_to_win_max_x - 2 * corner_radius as i32;
            p.draw_dot(
                right_dot_x,
                corner_radius as i32,
                corner_radius,
                title_color);
            // right window border decor down from top right circle
            p.draw_rect_filled(
                right_dot_x, corner_radius as i32,
                corner_radius + 1,
                w_fb.h - (corner_radius + padding as u32),
                title_color);

            let ww = w_fb.w - (1 * corner_radius + (3 * padding) as u32);
            let wh = w_fb.h - (2 * corner_radius + (2 * padding) as u32);

            p.push_add_offs(0, padding as i32 + 2 * corner_radius as i32);

            p.set_clip_rect(0, 0, ww, wh);
            child.draw(&self, &mut feedback[..], ww, wh, p);
            p.disable_clip_rect();
            p.pop_offs();
            p.pop_offs();
            for f in feedback.iter_mut() {
                f.offs(w_fb.x, w_fb.y);
            }
            self.feedback = feedback;
            self.win_feedback = w_fb;
        }
        p.done_cache_draw();
        p.pop_offs();
    }

    pub fn feedback_size(&self) -> (u32, u32) {
        (self.win_feedback.w, self.win_feedback.h)
    }

    pub fn add_layout(&mut self, s: Size, dir: BoxDir, border: i32, border_color: (u8, u8, u8, u8), c: &[usize]) -> usize {
        let id = self.widgets.len();
        self.widgets.push(Widget::Layout(id, s, Layout {
            dir,
            childs: c.to_vec(),
            border,
            border_color,
        }));
        self.does_need_redraw();
        id
    }

    pub fn add_canvas(&mut self, s: Size, cv: Canvas) -> usize {
        let id = self.widgets.len();
        self.widgets.push(Widget::Canvas(id, s, cv));
        self.does_need_redraw();
        id
    }

    pub fn add_label(&mut self, s: Size, l: Label) -> usize {
        let id = self.widgets.len();
        self.widgets.push(Widget::Label(id, s, l));
        self.does_need_redraw();
        id
    }

    pub fn needs_redraw(&self) -> bool { self.needs_redraw }
    pub fn does_need_redraw(&mut self) { self.needs_redraw = true; }

    pub fn get_label_text(&self, lblref: &str) -> Option<String> {
        for c in self.widgets.iter() {
            match c {
                Widget::Label(_, _, lbl) => {
                    if &lbl.lblref[..] == lblref {
                        return Some(lbl.text.clone());
                    }
                },
                _ => (),
            }
        }

        None
    }

    pub fn collect_label_texts(&self) -> std::vec::Vec<(String, String)> {
        let mut ret = vec![];
        for c in self.widgets.iter() {
            match c {
                Widget::Label(_, _, lbl) => {
                    ret.push((
                        lbl.lblref.to_string(),
                        lbl.text.to_string()));
                },
                _ => (),
            }
        }
        ret
    }

    pub fn set_label_text(&mut self, lblref: &str, text: String) {
        for c in self.widgets.iter_mut() {
            match c {
                Widget::Label(_, _, lbl) => {
                    if &lbl.lblref[..] == lblref {
                        lbl.text = text.clone();
                        self.needs_redraw = true;
                    }
                },
                _ => (),
            }
        }
    }

    pub fn collect_activated_child(&mut self) -> Option<String> {
        if let Some(idx) = self.activ_child {
            self.activ_child = None;
            match &self.widgets[idx] {
                Widget::Canvas(_, _, cv) => {
                    if let Some(sidx) = self.activ_sub_child {
                        self.activ_sub_child = None;
                        return Some(format!("{}:{}", cv.cbref, sidx));
                    }
                },
                Widget::Label(_, _, lbl) => { return Some(lbl.lblref.clone()); }
                _ => (),
            }
        }
        None
    }

    pub fn get_widget_at<P>(&self, x: u32, y: u32, p: P) -> Option<(usize, Option<usize>)>
        where P: Fn(&Label) -> bool {

        for (idx, fb) in self.feedback.iter().enumerate() {
            match &self.widgets[idx] {
                Widget::Canvas(_, _, lbl) => {
                    if fb.is_inside(x as u32, y as u32) {
                        if let Some(ref sfb) = fb.sub {
                            for c in sfb.iter() {
                                if c.is_inside(x as u32, y as u32) {
                                    return Some((fb.id, Some(c.id)));
                                }
                            }
                        }
                        return Some((fb.id, None));
                    }
                },
                Widget::Label(_, _, lbl) => {
                    if p(&lbl) {
                        if fb.is_inside(x as u32, y as u32) {
                            if let Some(ref sfb) = fb.sub {
                                for c in sfb.iter() {
                                    if c.is_inside(x as u32, y as u32) {
                                        return Some((fb.id, Some(c.id)));
                                    }
                                }
                            }
                            return Some((fb.id, None));
                        }
                    }
                }
                _ => ()
            }
        }
        return None;
    }

    pub fn handle_event(&mut self, ev: WindowEvent) -> bool {
        let mut handled = false;
        let r = match ev {
            WindowEvent::MousePos(x, y) => {
                if self.win_feedback.is_inside(x as u32, y as u32) {
                    handled = true;

                    let prev_hc     = self.hover_child;
                    let prev_hc_sub = self.hover_sub_child;

                    self.hover_child =
                        if let Some((child, sub)) =
                            self.get_widget_at(
                                x as u32,
                                y as u32,
                                |l| { l.clickable || l.editable })
                        {
                            self.hover_sub_child = sub;
                            Some(child)
                        } else {
                            self.hover_sub_child = None;
                            None
                        };

                    //d// println!("HOVER: {:?} | {:?}",
                    //d//          self.hover_child,
                    //d//          self.hover_sub_child);

                    prev_hc != self.hover_child
                    || prev_hc_sub != self.hover_sub_child
                } else {
                    false
                }
            },
            WindowEvent::Click(x, y)    => {
                if self.win_feedback.is_inside(x as u32, y as u32) {
                    handled = true;

                    let prev_fc = self.focus_child;
                    let prev_act_sub = self.activ_sub_child;

                    self.activ_child =
                        if let Some((child, sub)) =
                            self.get_widget_at(
                                x as u32, y as u32, |l: &Label| { l.clickable })
                        {
                            self.activ_sub_child = sub;
                            Some(child)
                        } else {
                            self.activ_sub_child = None;
                            None
                        };

                    self.focus_child =
                        if let Some((child, _)) =
                            self.get_widget_at(
                                x as u32, y as u32, |l: &Label| { l.editable })
                        { Some(child) }
                        else { None };

                    prev_fc != self.focus_child
                    || prev_act_sub != self.activ_sub_child
                } else {
                    false
                }
            },
            WindowEvent::TextInput(s)   => {
                if let Some(id) = self.focus_child {
                    match &mut self.widgets[id] {
                        Widget::Label(_, _, lbl) => {
                            let new = lbl.text.clone() + &s;
                            handled = true;
                            if let Ok(rx) = Regex::new(&lbl.edit_regex) {
                                if let Some(_) = rx.find(&new) {
                                    lbl.text = new;
                                }
                            }
                        }
                        _ => ()
                    }
                }
                true
            },
            WindowEvent::Backspace => {
                if let Some(id) = self.focus_child {
                    match &mut self.widgets[id] {
                        Widget::Label(_, _, lbl) => {
                            handled = true;
                            if !lbl.text.is_empty() {
                                lbl.text =
                                    lbl.text.chars()
                                        .take(lbl.text.chars().count() - 1)
                                        .collect();
                            }
                        }
                        _ => ()
                    }
                }
                true
            },
        };

        if r {
            self.does_need_redraw();
        }

        handled
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BoxDir {
    Vert(u32),
    Hori(u32),
}

#[derive(Debug, Clone)]
pub struct Layout {
    dir:            BoxDir,
    childs:         std::vec::Vec<usize>,
    border:         i32,
    border_color:   (u8, u8, u8, u8),
}

#[derive(Debug, Clone)]
pub struct Size {
    pub min_w: u32,
    pub w:     u32,
    pub min_h: u32,
    pub h:     u32,
    pub margin:u32,
}

impl Size {
    pub fn size(&self, max_w: u32, max_h: u32) -> (u32, u32) {
        let margin2 = self.margin * 2;
        let rw = p2r(max_w, self.w as i32) - margin2;
        let rh = p2r(max_h, self.h as i32) - margin2;
        (
            if rw < self.min_w { self.min_w } else { rw },
            if rh < self.min_h { self.min_h } else { rh },
        )
    }
}

#[derive(Debug, Clone)]
pub enum CanvasCmd {
    Circle(      Option<usize>, i32, i32, u32, (u8, u8, u8, u8)),
    CircleFilled(Option<usize>, i32, i32, u32, (u8, u8, u8, u8)),
    Line(        i32, i32,      i32, i32, u32, (u8, u8, u8, u8)),
    Rect(        Option<usize>, i32, i32, u32, u32, (u8, u8, u8, u8)),
    RectFilled(  Option<usize>, i32, i32, u32, u32, (u8, u8, u8, u8)),
    Text(        Option<usize>, i32, i32, u32, i32, String, FontSize, (u8, u8, u8, u8)),
}

#[derive(Debug, Clone)]
pub struct Canvas {
    cmds: std::vec::Vec<CanvasCmd>,
    cbref: String,
}

impl Canvas {
    pub fn new(cbref: String) -> Self {
        Self { cmds: vec![], cbref }
    }

    pub fn push(&mut self, cmd: CanvasCmd) {
        self.cmds.push(cmd);
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    lblref:     String,
    text:       String,
    wrap:       bool,
    align:      i32,
    editable:   bool,
    edit_regex: String,
    clickable:  bool,
    font_size:  FontSize,
    hlt_color:  (u8, u8, u8, u8),
    fg_color:   (u8, u8, u8, u8),
    bg_color:   (u8, u8, u8, u8),
}

#[allow(dead_code)]
impl Label {
    pub fn new(txt: &str, fg: (u8, u8, u8, u8), bg: (u8, u8, u8, u8)) -> Self {
        Self {
            lblref:    String::from(""),
            text:      txt.to_string(),
            align:     1,
            wrap:      false,
            editable:  false,
            edit_regex: String::from(".*"),
            clickable: false,
            hlt_color: (255 - fg.0, 255 - fg.1, 255 - fg.2, fg.3),
            fg_color:  fg,
            bg_color:  bg,
            font_size: FontSize::Normal,
        }
    }

    pub fn lblref(mut self, r: &str) -> Self {
        self.lblref = r.to_string();
        self
    }

    pub fn center_no_decor(mut self) -> Self {
        self.align = 2;
        self
    }

    pub fn center(mut self) -> Self {
        self.align = 0;
        self
    }

    pub fn normal_font(mut self) -> Self {
        self.font_size = FontSize::Normal;
        self
    }

    pub fn small_font(mut self) -> Self {
        self.font_size = FontSize::Small;
        self
    }

    pub fn right(mut self) -> Self {
        self.align = -1;
        self
    }

    pub fn left(mut self) -> Self {
        self.align = 1;
        self
    }

    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    pub fn editable(mut self, rx: &str) -> Self {
        self.editable  = true;
        self.edit_regex = String::from(rx);
        self
    }

    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
}
