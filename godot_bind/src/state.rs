use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use sscg::tree_painter::{DrawCmd, TreePainter, FontMetric};
use gdnative::*;
use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};
use crate::wl_gd_mod_resolver::*;

#[derive(Debug, Clone)]
pub struct FontHolder {
    pub main_font: DynamicFont,
}

impl FontMetric for FontHolder {
    fn text_size(&self, text: &str) -> (u32, u32) {
        let s = self.main_font.get_string_size(GodotString::from_str(text));
        (s.x as u32, s.y as u32)
    }
}

#[derive(Clone)]
pub struct SSCGState {
    pub fonts: Rc<FontHolder>,
    pub tp:    TreePainter,
    pub v:     std::vec::Vec<DrawCmd>,
    pub temp_stations: std::vec::Vec<(i32, i32)>,
    pub update_stations: bool,
    pub wlctx: EvalContext,
    pub state: VVal,
}

// XXX: This is safe as long as it is only accessed from the
//      Godot main thread. If there are going to be multiple
//      threads, we will probably need to split it up anyways.
unsafe impl Send for SSCGState { }

impl SSCGState {
    pub fn new(fh: Rc<FontHolder>, cmds: std::vec::Vec<DrawCmd>) -> Self {
        dbg!("INIT SSCGState");
        let genv = GlobalEnv::new_default();
        genv.borrow_mut().set_resolver(
            Rc::new(RefCell::new(GodotModuleResolver::new())));
        let tp = TreePainter::new(fh.clone());
        Self {
            fonts: fh,
            v: cmds,
            temp_stations: vec![(1, 1), (900, 500)],
            update_stations: true,
            tp,
            wlctx: EvalContext::new(genv),
            state: VVal::Nul,
        }
    }

    pub fn setup_wlambda(&mut self) {
        match self.wlctx.eval("!@import main main; main:init[]") {
            Ok(state) => {
                self.state = state.clone();
                dbg!("SET STATE INIT!");
            },
            Err(e) => { godot_print!("main.wl error: {:?}", e); }
        }
    }
}

lazy_static! {
    pub static ref SSCG : Arc<Mutex<Option<SSCGState>>> =
        Arc::new(Mutex::new(None));
}
