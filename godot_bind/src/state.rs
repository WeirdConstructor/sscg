use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use sscg::tree_painter::{DrawCmd, TreePainter, FontMetric};
use sscg::wlambda_api::WindowManager;
use sscg::wlambda_api::window_manager_wlambda_obj;
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
    pub fonts:           Rc<FontHolder>,
    pub tp:              TreePainter,
    pub temp_stations:   std::vec::Vec<(i32, i32)>,
    pub update_stations: bool,
    pub wlctx:           EvalContext,
    pub cb_arrived:      VVal,
    pub state:           VVal,
    pub cmd_queue:       std::vec::Vec<VVal>,
    pub wm:              Rc<RefCell<WindowManager>>,
}

// XXX: This is safe as long as it is only accessed from the
//      Godot main thread. If there are going to be multiple
//      threads, we will probably need to split it up anyways.
unsafe impl Send for SSCGState { }

impl SSCGState {
    pub fn new(fh: Rc<FontHolder>) -> Self {
        dbg!("INIT SSCGState");
        let genv = GlobalEnv::new_default();
        genv.borrow_mut().set_resolver(
            Rc::new(RefCell::new(GodotModuleResolver::new())));

        let wm = Rc::new(RefCell::new(WindowManager::new()));

        let mut sscg_wl_mod = SymbolTable::new();
        sscg_wl_mod.set("win", window_manager_wlambda_obj(wm.clone()));
//        sscg_wl_mod.set("win",  WindowManagerWlWrapper::vval_from(s_wm.clone()));
        genv.borrow_mut().set_module("sscg", sscg_wl_mod);

        let tp = TreePainter::new(fh.clone());
        Self {
            tp,
            wm,
            fonts:           fh,
            temp_stations:   vec![(1, 1), (900, 500)],
            update_stations: true,
            wlctx:           EvalContext::new(genv),
            cb_arrived:      VVal::Nul,
            state:           VVal::Nul,
            cmd_queue:       std::vec::Vec::new(),
        }
    }

    pub fn call_cb(&mut self, name: &str, args: &[VVal]) -> VVal {
        let cb =
            match self.wlctx.get_global_var(name) {
                None => {
                    godot_print!(
                        "No such callback {} (args: {:?})!",
                        name, args);
                    return VVal::Nul;
                },
                Some(cb) => cb,
            };
        match self.wlctx.call(&cb, args) {
            Err(e) => {
                godot_print!("Error on {} (args: {:?}): {}", name, args, e);
                VVal::Nul
            },
            Ok(v) => v,
        }
    }

    pub fn setup_wlambda(&mut self) {
        println!("START WLAM");
        match self.wlctx.eval(r"
            !@import main main;
            !:global on_arrived = main:on_arrived;
            !:global on_tick    = main:on_tick;
            !:global STATE      = main:STATE;
            main:init[]")
        {
            Ok(state) => {
                self.state = state.clone();
                dbg!("SET STATE INIT!");
            },
            Err(e) => { godot_print!("main.wl error: {:?}", e); }
        }

        self.cb_arrived =
            self.wlctx.get_global_var("on_arrived").unwrap_or(VVal::Nul);
    }
}

#[macro_export]
macro_rules! lock_sscg {
    ($var: ident) => {
        let mut sscg_lock = SSCG.lock().unwrap();
        let $var = sscg_lock.as_mut().unwrap();
    }
}

lazy_static! {
    pub static ref SSCG : Arc<Mutex<Option<SSCGState>>> =
        Arc::new(Mutex::new(None));
}
