use std::sync::{Arc, Mutex};
use std::rc::Rc;
//use std::cell::RefCell;
use sscg::tree_painter::{DrawCmd, TreePainter, FontMetric};
use gdnative::*;

pub struct FontHolder {
    pub main_font: DynamicFont,
}

impl FontMetric for FontHolder {
    fn text_size(&self, _text: &str) -> (u32, u32) {
        (0, 0)
    }
}

pub struct SSCGState {
    pub fonts: Rc<FontHolder>,
    pub tp:    TreePainter,
    pub v:     std::vec::Vec<DrawCmd>,
    pub temp_stations: std::vec::Vec<(i32, i32)>,
    pub update_stations: bool,
}

// XXX: This is safe as long as it is only accessed from the
//      Godot main thread. If there are going to be multiple
//      threads, we will probably need to split it up anyways.
unsafe impl Send for SSCGState { }

lazy_static! {
    pub static ref SSCG : Arc<Mutex<Option<SSCGState>>> =
        Arc::new(Mutex::new(None));
}
