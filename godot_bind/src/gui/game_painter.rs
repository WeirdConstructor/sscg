#[derive(Debug, Clone, PartialEq, Hash, Copy)]
pub enum FontSize {
    Normal,
    Small,
}

pub trait GamePainter {
    fn push_offs(&mut self, xo: i32, yo: i32);
    fn push_add_offs(&mut self, xo: i32, yo: i32);

    fn declare_cache_draw(&mut self, xo: i32, yo: i32, w: u32, h: u32, id: usize, repaint: bool);
    fn done_cache_draw(&mut self);

    fn pop_offs(&mut self);
    fn get_screen_pos(&self, xo: i32, yo: i32) -> (i32, i32);
    fn disable_clip_rect(&mut self);
    fn set_clip_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32);
    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                 color: (u8, u8, u8, u8));
    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32,
                        color: (u8, u8, u8, u8));
    fn draw_texture(&mut self, idx: usize, xo: i32, yo: i32, w: u32, h: u32);
    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32,
                 color: (u8, u8, u8, u8));
    fn text_size(&mut self, txt: &str, fs: FontSize) -> (u32, u32);
    fn texture_crop(&mut self, idx: usize, xo: i32, yo: i32, w: u32, h: u32);
    fn texture(&mut self, idx: usize, xo: i32, yo: i32, centered: bool);
    fn texture_size(&mut self, idx: usize) -> (u32, u32);
    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32,
                 fg: (u8, u8, u8, u8),
                 bg: Option<(u8, u8, u8, u8)>,
                 align: i32,
                 txt: &str,
                 fs: FontSize);
}

