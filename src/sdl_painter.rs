use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::rc::Rc;
use std::cell::RefCell;
use crate::logic::GamePainter;
use sdl2::gfx::primitives::{DrawRenderer};

pub struct SDLPainter<'a, 'b> {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub font: Rc<RefCell<sdl2::ttf::Font<'a, 'b>>>,
    pub font_h: i32,
    pub offs_stack: std::vec::Vec<(i32, i32)>,
    pub offs: (i32, i32),
}

impl<'a, 'b> SDLPainter<'a, 'b> {
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    pub fn get_font_h(&mut self) -> i32 {
        if self.font_h == 0 {
            let (_w, h) = self.text_size("M");
            self.font_h = h as i32;
        }
        self.font_h
    }

    pub fn text_size(&mut self, txt: &str) -> (u32, u32) {
        if txt.is_empty() {
            (0, self.get_font_h() as u32)
        } else {
            self.font.borrow().size_of(txt).unwrap_or((0, 0))
        }
    }

    pub fn done(&mut self) {
        self.canvas.present();
    }
}

impl<'a, 'b> GamePainter for SDLPainter<'a, 'b> {
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

    fn get_sprite_size(&self, _id: usize) -> (u32, u32) {
        (0, 0)
    }
    fn draw_sprite_ex(&mut self, _xo: i32, _yo: i32, _w: u32, _h: u32,
                      _id: usize, _angle: f64, _flip_h: bool, _flip_v: bool) {
    }

    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8)) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.draw_rect(Rect::new(xo + self.offs.0, yo + self.offs.1, w, h))
            .expect("drawing rectangle");
    }

    fn disable_clip_rect(&mut self) {
        self.canvas.set_clip_rect(None);
    }
    fn set_clip_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32) {
        self.canvas.set_clip_rect(
            Rect::new(xo + self.offs.0, yo + self.offs.1, w, h));
    }

    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8)) {
        self.canvas.set_draw_color(Color::from(color));
        self.canvas.fill_rect(Rect::new(xo + self.offs.0, yo + self.offs.1, w, h))
            .expect("filling rectangle");
    }

    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.canvas.filled_circle(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            r as i16,
            Color::from(color)).expect("drawing filled_circle");
    }
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8)) {
        self.canvas.circle(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            r as i16,
            Color::from(color)).expect("drawing circle");
    }
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32, color: (u8, u8, u8, u8)) {
        self.canvas.thick_line(
            (self.offs.0 + xo ) as i16,
            (self.offs.1 + yo ) as i16,
            (self.offs.0 + x2o) as i16,
            (self.offs.1 + y2o) as i16,
            t as u8,
            Color::from(color)).expect("drawing thick_line");
    }
    fn text_size(&mut self, txt: &str) -> (u32, u32) {
        self.text_size(txt)
    }

    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32, fg: (u8, u8, u8, u8),
                 bg: Option<(u8, u8, u8, u8)>, align: i32, txt: &str) {
        if let Some(c) = bg {
            let h = self.get_font_h();
            draw_bg_text(
                &mut self.canvas,
                &mut *self.font.borrow_mut(),
                fg.into(),
                c.into(),
                self.offs.0 + xo,
                self.offs.1 + yo,
                max_w as i32,
                h,
                align,
                txt);
        } else {
            draw_text(
                &mut *self.font.borrow_mut(),
                fg.into(),
                &mut self.canvas,
                self.offs.0 + xo,
                self.offs.1 + yo,
                max_w as i32,
                align,
                txt);
        }
    }
}

fn draw_text(font: &mut sdl2::ttf::Font, color: Color,
             canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
             x: i32, y: i32, max_w: i32, align: i32, txt: &str) {
    if txt.is_empty() { return; }

    let txt_crt = canvas.texture_creator();

    let sf  = font.render(txt).blended(color).map_err(|e| e.to_string()).unwrap();
    let txt = txt_crt.create_texture_from_surface(&sf).map_err(|e| e.to_string()).unwrap();
    let tq  = txt.query();

    let xo = if align == 2
             || align == 0 { (max_w - (tq.width as i32)) / 2 }
        else if align < 0  { max_w - (tq.width as i32) }
        else { 0 };

    let w : i32 = if max_w < (tq.width as i32) { max_w } else { tq.width as i32 };

    let xo = if xo < 0 { 0 } else { xo };

    canvas.copy(
        &txt,
        Some(Rect::new(0,      0, w as u32, tq.height)),
        Some(Rect::new(x + xo, y, w as u32, tq.height))
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
                align: i32,
                txt: &str) {

    canvas.set_draw_color(bg_color);
    canvas.fill_rect(Rect::new(x, y, max_w as u32, h as u32))
        .expect("filling rectangle");
    draw_text(font, color, canvas, x, y, max_w, align, txt);
}

