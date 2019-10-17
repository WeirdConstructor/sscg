pub trait GamePainter {
    fn push_offs(&mut self, xo: i32, yo: i32);
    fn push_add_offs(&mut self, xo: i32, yo: i32);
    fn pop_offs(&mut self);
    fn get_screen_pos(&self, xo: i32, yo: i32) -> (i32, i32);
    fn draw_rect(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8));
    fn draw_rect_filled(&mut self, xo: i32, yo: i32, w: u32, h: u32, color: (u8, u8, u8, u8));
    fn draw_dot(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_circle(&mut self, xo: i32, yo: i32, r: u32, color: (u8, u8, u8, u8));
    fn draw_line(&mut self, xo: i32, yo: i32, x2o: i32, y2o: i32, t: u32, color: (u8, u8, u8, u8));
    fn draw_text(&mut self, xo: i32, yo: i32, max_w: u32, fg: (u8, u8, u8, u8), bg: Option<(u8, u8, u8, u8)>, txt: &str);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SystemObject {
    Station,
    AsteroidField,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Entity {
    pub typ: SystemObject,
    pub x:   i32,
    pub y:   i32,
    draw_pos: (i32, i32),
    is_highlighted: bool,
}

impl Entity {
    pub fn new(typ: SystemObject) -> Self {
        Entity {
            typ,
            draw_pos: (0, 0),
            x: 0,
            y: 0,
            is_highlighted: false
        }
    }
    fn draw<P>(&mut self, p: &mut P) where P: GamePainter {
        if self.is_highlighted {
            p.draw_circle(0, 0, 8, (255, 0, 0, 255));
        }
        match self.typ {
            SystemObject::Station => {
                p.draw_dot(0, 0, 4, (0, 190, 0, 255));
            },
            SystemObject::AsteroidField => {
                p.draw_dot(0, 0, 2, (190, 190, 190, 255));
            },
        }
        self.draw_pos = p.get_screen_pos(0, 0);
    }

    fn set_highlight(&mut self, h: bool) {
        self.is_highlighted = h;
    }
}

#[derive(Debug, Clone)]
pub struct System {
    x:   i32,
    y:   i32,
    objects: std::vec::Vec<Entity>,
}

impl System {
    pub fn new(x: i32, y: i32) -> Self {
        System { x, y, objects: std::vec::Vec::new() }
    }

    pub fn add(&mut self, x: i32, y: i32, mut e: Entity) {
        e.x = x;
        e.y = y;
        self.objects.push(e);
    }

    pub fn clear_entity_highlights(&mut self) {
        for e in self.objects.iter_mut() {
            e.set_highlight(false);
        }
    }

    pub fn try_highlight_entity_close_to(&mut self, x_screen: i32, y_screen: i32) {
        self.clear_entity_highlights();
        if let Some(e) = self.get_entity_close_to(x_screen, y_screen) {
            e.set_highlight(true);
        }
    }

    pub fn get_entity_close_to(&mut self, x_screen: i32, y_screen: i32) -> Option<&mut Entity> {
        let mut closest_i : i32 = -1;
        let mut last_dist : i32 = 99999;

        for (i, ent) in self.objects.iter().enumerate() {
            let d : i32 = (ent.draw_pos.0 - x_screen).pow(2)
                        + (ent.draw_pos.1 - y_screen).pow(2);
            println!("TEST: {:?}", ent);
            if d < last_dist {
                last_dist = d;
                closest_i = i as i32;
            println!("TEST: {} {} => {}", ent.draw_pos.0, x_screen, last_dist);
            }
        }

        if last_dist < 20_i32.pow(2) {
            return self.objects.get_mut(closest_i as usize);
        }
        return None;
    }

    pub fn draw<P>(&mut self, p: &mut P) where P: GamePainter {
        let cell_count = 10;
        let cell_size  = 48;
        let w = cell_count * cell_size;
        p.draw_rect_filled(
            0, 0,
            cell_size * cell_count, cell_size * cell_count,
            (0, 0, 50, 255));
        for h_line_y in 0..11 {
            p.draw_line(0, h_line_y * 48, (w - 1) as i32, h_line_y * 48, 1, (200, 0, 0, 255));
        }
        for v_line_x in 0..11 {
            p.draw_line(v_line_x * 48, 0, v_line_x * 48, (w - 1) as i32, 1, (200, 0, 0, 255));
        }

        for ent in self.objects.iter_mut() {
            p.push_add_offs(ent.x, ent.y);
            ent.draw(p);
            p.pop_offs();
        }
    }
}
