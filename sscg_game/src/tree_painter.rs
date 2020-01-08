use std::rc::Rc;
use crate::logic::GamePainter;

pub use crate::logic::FontSize;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum DrawCmd {
    ClipRectOff,
    ClipRect     { x: i32, y: i32, w: u32, h: u32 },
    Rect         { x: i32, y: i32, w: u32, h: u32, color: (u8, u8, u8, u8) },
    FilledRect   { x: i32, y: i32, w: u32, h: u32, color: (u8, u8, u8, u8) },
    Circle       { x: i32, y: i32, r: u32, color: (u8, u8, u8, u8) },
    FilledCircle { x: i32, y: i32, r: u32, color: (u8, u8, u8, u8) },
    Line         { x: i32, y: i32, x2: i32, y2: i32, t: u32, color: (u8, u8, u8, u8) },
    TextureCrop  { txt_idx: usize, x: i32, y: i32, w: u32, h: u32, },
    Texture      { txt_idx: usize, x: i32, y: i32, centered: bool },
    Text         { txt: String, align: i32, color: (u8, u8, u8, u8), x: i32, y: i32, w: u32, fs: FontSize },
    CacheDraw    { w: u32, h: u32, id: usize, cmds: std::vec::Vec<DrawCmd> },
    DrawCache    { x: i32, y: i32, w: u32, h: u32, id: usize },
}

pub trait FontMetric {
    fn text_size(&self, text: &str, fs: FontSize) -> (u32, u32);
}

#[derive(Clone)]
pub struct TreePainter {
    text_metric:        Rc<dyn FontMetric>,
    cache_tmp_cmds:     Option<(i32, i32, u32, u32, usize, std::vec::Vec<DrawCmd>)>,
    cmds:               std::vec::Vec<DrawCmd>,
    offs_stack:         std::vec::Vec<(i32, i32)>,
    offs:               (i32, i32),
}

impl TreePainter {
    pub fn new(metric: Rc<dyn FontMetric>) -> Self {
        Self {
            text_metric:    metric,
            cmds:           std::vec::Vec::new(),
            offs_stack:     std::vec::Vec::new(),
            offs:           (0, 0),
            cache_tmp_cmds: None,
        }
    }

    pub fn clear_cmds(&mut self) { self.cmds.clear(); }
    pub fn ref_cmds(&self) -> &[DrawCmd] { &self.cmds }

    pub fn consume_cmds(&mut self) -> std::vec::Vec<DrawCmd> {
        std::mem::replace(&mut self.cmds, std::vec::Vec::new())
    }
}

impl GamePainter for TreePainter {
    fn push_offs(&mut self, xo: i32, yo: i32) {
        self.offs_stack.push(self.offs);
        self.offs = (xo, yo);
    }

    fn push_add_offs(&mut self, xo: i32, yo: i32) {
        self.push_offs(xo + self.offs.0, yo + self.offs.1);
    }

    fn pop_offs(&mut self) {
        if !self.offs_stack.is_empty() {
            self.offs = self.offs_stack.pop().unwrap();
        }
    }

    fn get_screen_pos(&self, xo: i32, yo: i32) -> (i32, i32) {
        ((self.offs.0 + xo) as i32,
         (self.offs.1 + yo) as i32)
    }

    fn declare_cache_draw(&mut self, xo: i32, yo: i32, w: u32, h: u32, id: usize, repaint: bool) {
        let x = self.offs.0 + xo;
        let y = self.offs.1 + yo;
        if !repaint {
            self.cmds.push(DrawCmd::DrawCache {
                x, y, w, h, id });
            return;
        }

        self.cache_tmp_cmds =
            Some((x, y, w, h, id,
                  std::mem::replace(
                      &mut self.cmds, std::vec::Vec::new())));
        self.push_offs(0, 0);
    }

    fn done_cache_draw(&mut self) {
        let prev_cmds =
            std::mem::replace(&mut self.cache_tmp_cmds, None);
        if let Some(prev_cmds) = prev_cmds {
            let x  = prev_cmds.0;
            let y  = prev_cmds.1;
            let w  = prev_cmds.2;
            let h  = prev_cmds.3;
            let id = prev_cmds.4;
            let cached_cmds = std::mem::replace(&mut self.cmds, prev_cmds.5);
            self.cmds.push(DrawCmd::CacheDraw {
                w, h, id, cmds: cached_cmds,
            });
            self.cmds.push(DrawCmd::DrawCache {
                x, y, w, h, id });
            self.pop_offs();
        }
    }

    fn disable_clip_rect(&mut self) {
        self.cmds.push(DrawCmd::ClipRectOff);
    }

    fn set_clip_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32) {
        self.cmds.push(DrawCmd::ClipRect {
            x: self.offs.0 + xo,
            y: self.offs.1 + yo,
            w, h
        });
    }

    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                 color: (u8, u8, u8, u8)) {
        self.cmds.push(DrawCmd::Rect {
            x: self.offs.0 + xo,
            y: self.offs.1 + yo,
            w, h,
            color,
        });
    }
    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                        color: (u8, u8, u8, u8)) {
        self.cmds.push(DrawCmd::FilledRect {
            x: self.offs.0 + xo,
            y: self.offs.1 + yo,
            w, h,
            color,
        });
    }
    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.cmds.push(DrawCmd::FilledCircle {
            x: self.offs.0 + xo,
            y: self.offs.1 + yo,
            r,
            color,
        });
    }
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.cmds.push(DrawCmd::Circle {
            x: self.offs.0 + xo,
            y: self.offs.1 + yo,
            r,
            color,
        });
    }
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32,
                 color: (u8, u8, u8, u8)) {
        self.cmds.push(DrawCmd::Line {
            x:  self.offs.0 + xo,
            y:  self.offs.1 + yo,
            x2: self.offs.0 + x2o,
            y2: self.offs.1 + y2o,
            t,
            color,
        });
    }
    fn text_size(&mut self, txt: &str, fs: FontSize) -> (u32, u32) {
        self.text_metric.text_size(txt, fs)
    }
    fn texture_crop(&mut self, idx: usize, xo: i32, yo: i32, w: u32, h: u32) {
        self.cmds.push(DrawCmd::TextureCrop {
            txt_idx: idx,
            x:  self.offs.0 + xo,
            y:  self.offs.1 + yo,
            w,
            h,
        });
    }
    fn texture_size(&mut self, _idx: usize) -> (u32, u32) {
        (0, 0)
    }
    fn texture(&mut self, idx: usize, xo: i32, yo: i32, centered: bool) {
        self.cmds.push(DrawCmd::Texture {
            txt_idx: idx,
            x:  self.offs.0 + xo,
            y:  self.offs.1 + yo,
            centered,
        });
    }
    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32,
                 fg: (u8, u8, u8, u8),
                 bg: Option<(u8, u8, u8, u8)>,
                 align: i32,
                 txt: &str,
                 fs: FontSize) {
        if let Some(c) = bg {
            let fm = self.text_size(txt, fs);
            self.cmds.push(DrawCmd::FilledRect {
                x:      self.offs.0 + xo,
                y:      self.offs.1 + yo,
                w:      max_w,
                h:      fm.1,
                color:  c,
            });
        }

        self.cmds.push(DrawCmd::Text {
            x:      self.offs.0 + xo,
            y:      self.offs.1 + yo,
            w:      max_w,
            color:  fg,
            txt:    txt.to_string(),
            fs,
            align,
        });
    }
}
