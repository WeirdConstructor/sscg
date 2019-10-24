use crate::logic::GamePainter;

#[derive(Debug, Clone)]
pub enum Widget {
    Layout(usize, Size, Layout),
    Label(usize, Size, Label),
}

impl Widget {
    pub fn id(&self) -> usize {
        match self {
            Widget::Layout(id, _, _) => *id,
            Widget::Label(id, _, _)  => *id,
        }
    }
    pub fn calc_feedback<P>(&self, max_w: u32, max_h: u32, p: &mut P) -> WidgetFeedback where P: GamePainter {
        let pos = p.get_screen_pos(0, 0);
        let (id, (mw, mh)) = match self {
            Widget::Layout(id, l, _) => (*id, l.size(max_w, max_h)),
            Widget::Label(id, l, _)  => (*id, l.size(max_w, max_h)),
        };
        WidgetFeedback {
            id,
            x: pos.0 as u32,
            y: pos.1 as u32,
            w: mw,
            h: mh,
        }
    }

    pub fn draw<P>(&self, win: &Window, fb: &mut [WidgetFeedback], max_w: u32, max_h: u32, p: &mut P) -> (u32, u32)
        where P: GamePainter {

        let w_fb = self.calc_feedback(max_w, max_h, p);
        fb[self.id()] = w_fb;
        let (mw, mh) = (w_fb.w, w_fb.h);
        match self {
            Widget::Layout(_id, _size, c) => {
                match c.dir {
                    BoxDir::Vert => {
                        let mut offs = 0;
                        p.push_add_offs(0, offs);
                        for c_id in c.childs.iter() {
                            let (_w, h) =
                                win.widgets[*c_id].draw(
                                    win, fb, mw, mh, p);
                            offs += h as i32;
                            p.pop_offs();
                            p.push_add_offs(0, offs);
                        }
                        p.pop_offs();
                    },
                    BoxDir::Hori => {
                        let mut offs = 0;
                        p.push_add_offs(offs, 0);
                        for c_id in c.childs.iter() {
                            let (w, _h) =
                                win.widgets[*c_id].draw(
                                    win, fb, mw, mh, p);
                            offs += w as i32;
                            p.pop_offs();
                            p.push_add_offs(offs, 0);
                        }
                        p.pop_offs();
                    },
                }
            },
            Widget::Label(_id, _size, lbl) => {
                let txt = lbl.text.clone();
                let (tw, _th) = p.text_size(&txt);
                if lbl.wrap && tw > mw {
                    let mut line = String::from("");
                    let mut y = 0;
                    for c in txt.chars() {
                        line.push(c);
                        let (tw, th) = p.text_size(&line);
                        if tw > mw {
                            line.pop();
                            p.draw_text(
                                0, y, mw,
                                lbl.fg_color, Some(lbl.bg_color), lbl.align, &line);
                            line = String::from("");
                            line.push(c);
                            y += th as i32;
                        }
                    }

                    if line.len() > 0 {
                        p.draw_text(
                            0, y, mw,
                            lbl.fg_color, Some(lbl.bg_color), lbl.align, &line);
                    }
                } else {
                    p.draw_text(
                        0, 0,
                        mw, lbl.fg_color, Some(lbl.bg_color), lbl.align, &lbl.text);
                }
            },
        }
        (w_fb.w, w_fb.h)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WidgetFeedback {
    id: usize,
    x:  u32,
    y:  u32,
    w:  u32,
    h:  u32,
}

impl WidgetFeedback {
    pub fn new() -> Self {
        Self {
            id: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    pub fn is_inside(&self, x: u32, y: u32) -> bool {
           x >= self.x && x <= (self.x + self.w)
        && y >= self.y && y <= (self.y + self.h)
    }
}

pub struct Window {
    pub title:    String,
    widgets:      std::vec::Vec<Widget>,
    feedback:     std::vec::Vec<WidgetFeedback>,
    pub child:    usize,
    focus_child:  Option<usize>,
    hover_child:  Option<usize>,
    activ_child:  Option<usize>,
    pub x:        u32,
    pub y:        u32,
    pub w:        u32,
    pub h:        u32,
    pub min_w:    u32,
    pub min_h:    u32,
    needs_redraw: bool,
    win_feedback:     WidgetFeedback,
}

/// All values are in 0.1% scale. that means, to represent 100% you have to
/// supply 1000 to ratio to get the full value.
fn p2r(value: u32, ratio: u32) -> u32 {
    (value * ratio) / 1000
}

pub enum WindowEvent {
    MousePos(i32, i32),
    Click(i32, i32),
    TextInput(String),
    Backspace,
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            widgets: std::vec::Vec::new(),
            feedback: std::vec::Vec::new(),
            child: 0,
            focus_child: None,
            hover_child: None,
            activ_child: None,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            min_w: 0,
            min_h: 0,
            needs_redraw: false,
            win_feedback: WidgetFeedback::new(),
        }
    }

    pub fn draw<P>(&mut self, max_w: u32, max_h: u32, p: &mut P)
        where P: GamePainter {

        let mut feedback = std::vec::Vec::new();
        feedback.resize(self.widgets.len(), WidgetFeedback::new());
        let child    = &self.widgets[self.child];

        let mut w_fb = WidgetFeedback::new();

        w_fb.x = p2r(max_w, self.x);
        w_fb.y = p2r(max_h, self.y);
        w_fb.w = p2r(max_w, self.w);
        w_fb.h = p2r(max_h, self.h);

        let padding : i32 = 4;
        let ww = w_fb.w - ((2 * padding) as u32);
        let wh = w_fb.h - ((3 * padding) as u32);

        p.push_offs(
            padding + w_fb.x as i32,
            padding + w_fb.y as i32);

        let ts = p.text_size(&self.title);
        p.draw_rect_filled(
            -padding,
            -padding,
            w_fb.w + 2 * (padding as u32),
            w_fb.h + 2 * (padding as u32),
            (55, 0, 44, 255));
        let title_height = (ts.1 + 4) / 2;
        let gclr = (255, 128, 128, 255);
        p.draw_dot(
            title_height as i32,
            title_height as i32,
            title_height,
            gclr);
        p.draw_rect_filled(
            title_height as i32, 0,
            2 * title_height + 1, 2 * title_height + 1,
            gclr);
        p.draw_text(
            3 * title_height as i32 + 3,
            2,
            ts.0,
            gclr, None, 1, &self.title);
        p.draw_rect_filled(
            title_height as i32, 0,
            2 * title_height + 1, 2 * title_height + 1,
            gclr);

        p.push_add_offs(0, padding as i32 + 2 * title_height as i32);

        child.draw(&self, &mut feedback[..], ww, wh, p);
        p.pop_offs();
        p.pop_offs();
        p.pop_offs();
        self.feedback = feedback;
        self.win_feedback = w_fb;
    }

    pub fn add_layout(&mut self, s: Size, dir: BoxDir, c: &[usize]) -> usize {
        let id = self.widgets.len();
        self.widgets.push(Widget::Layout(id, s, Layout {
            dir,
            childs: c.to_vec(),
        }));
        id
    }

    pub fn add_label(&mut self, s: Size, l: Label) -> usize {
        let id = self.widgets.len();
        self.widgets.push(Widget::Label(id, s, l));
        id
    }

    pub fn needs_redraw(&self) -> bool { self.needs_redraw }

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
            match &self.widgets[idx] {
                Widget::Label(_, _, lbl) => { return Some(lbl.lblref.clone()); }
                _ => (),
            }
        }
        None
    }

    pub fn handle_event(&mut self, ev: WindowEvent) -> bool {
        match ev {
            WindowEvent::MousePos(x, y) => {
                if self.win_feedback.is_inside(x as u32, y as u32) {
                    println!("MOUSE INSIDE WIN");
                    true
                } else {
                    false
                }
            },
            WindowEvent::Click(_x, _y)    => {
                // set self.activ_child ...
                true
            },
            WindowEvent::TextInput(s)   => { true },
            WindowEvent::Backspace      => { true },
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum BoxDir {
    Vert,
    Hori,
}

#[derive(Debug, Clone)]
pub struct Layout {
    dir:        BoxDir,
    childs:     std::vec::Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
enum HAlign {
    Left,
    Center,
    Right,
}

impl HAlign {
    fn xoffs(&self, tw: u32, mw: u32) -> i32 {
        (match self {
            HAlign::Left   => 0,
            HAlign::Right  => mw - tw,
            HAlign::Center => (mw - tw) / 2,
        }) as i32
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub min_w: u32,
    pub w:     u32,
    pub min_h: u32,
    pub h:     u32,
}

impl Size {
    pub fn size(&self, max_w: u32, max_h: u32) -> (u32, u32) {
        let rw = p2r(max_w, self.w);
        let rh = p2r(max_h, self.h);
        (
            if rw < self.min_w { self.min_w }
            else { rw },
            if rh < self.min_h { self.min_h }
            else { rh },
        )
    }
}


#[derive(Debug, Copy, Clone)]
pub enum LabelStyle {
}

#[derive(Debug, Clone)]
pub struct Label {
    lblref:     String,
    text:       String,
    wrap:       bool,
    align:      i32,
    editable:   bool,
    clickable:  bool,
    fg_color:   (u8, u8, u8, u8),
    bg_color:   (u8, u8, u8, u8),
}

impl Label {
    pub fn new(txt: &str, fg: (u8, u8, u8, u8), bg: (u8, u8, u8, u8)) -> Self {
        Self {
            lblref:    String::from(""),
            text:      txt.to_string(),
            align:     1,
            wrap:      false,
            editable:  false,
            clickable: false,
            fg_color:  fg,
            bg_color:  bg,
        }
    }

    pub fn lblref(mut self, r: &str) -> Self {
        self.lblref = r.to_string();
        self
    }

    pub fn center(mut self) -> Self {
        self.align = 0;
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

    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }
}
