use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;
use sscg::tree_painter::{TreePainter, FontMetric, FontSize};
use sscg::wlambda_api::WindowManager;
use sscg::wlambda_api::window_manager_wlambda_obj;
use gdnative::*;
use wlambda::{VVal, Env, GlobalEnv, EvalContext, SymbolTable};
use wlambda::set_vval_method;
use crate::voxeltree_wlambda::*;
use crate::wl_gd_mod_resolver::*;

#[derive(Debug, Clone)]
pub struct FontHolder {
    pub main_font:  DynamicFont,
    pub small_font: DynamicFont,
}

impl FontMetric for FontHolder {
    fn text_size(&self, text: &str, fs: FontSize) -> (u32, u32) {
        let s = match fs {
            FontSize::Normal => self.main_font.get_string_size(GodotString::from_str(text)),
            FontSize::Small  => self.small_font.get_string_size(GodotString::from_str(text)),
        };
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
    pub state:           VVal,
    pub cmd_queue:       Rc<RefCell<std::vec::Vec<VVal>>>,
    pub wm:              Rc<RefCell<WindowManager>>,
    pub vox_painters:    Rc<RefCell<std::vec::Vec<Rc<RefCell<VoxelPainter>>>>>,
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

        let cmd_queue = Rc::new(RefCell::new(std::vec::Vec::new()));

        let o = VVal::map();
        set_vval_method!(o, cmd_queue, cmd, Some(2), Some(2), env, _argc, {
            let v = VVal::vec();
            v.push(env.arg(0));
            v.push(env.arg(1));
            cmd_queue.borrow_mut().push(v);
            Ok(VVal::Nul)
        });
        let _cmd_queue = cmd_queue.clone();
        set_vval_method!(o, _cmd_queue, read_savegame, Some(1), Some(1), env, _argc, {
            let filename = env.arg(0).s_raw();

            let savegame_url = format!("user://{}.json", filename);

            let mut f = File::new();
            match f.open(GodotString::from_str(savegame_url.clone()), 1) {
                Ok(_) => {
                    let txt = f.get_as_text().to_string();
                    match VVal::from_json(&txt) {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            Ok(VVal::err_msg(
                                &format!("Couldn't load game '{}': {:?}",
                                         savegame_url, e)))
                        }
                    }
                },
                Err(e) => {
                    Ok(VVal::err_msg(
                        &format!("Couldn't load game '{}': {:?}",
                                 savegame_url, e)))
                }
            }
        });
        set_vval_method!(o, _cmd_queue, write_savegame, Some(2), Some(2), env, _argc, {
            let filename = env.arg(0).s_raw();
            let state    = env.arg(1);

            let savegame_url = format!("user://{}.json", filename);

            let mut f = File::new();
            match f.open(GodotString::from_str(savegame_url.clone()), 2) {
                Ok(_) => {
                    match state.to_json(false) {
                        Ok(s) => {
                            f.store_string(GodotString::from_str(s));
                        },
                        Err(e) => {
                            return Ok(VVal::err_msg(
                                        &format!("Couldn't save game '{}': {:?}", savegame_url, e)));
                        },
                    }
                },
                Err(e) => {
                    f.close();
                    return Ok(VVal::err_msg(
                            &format!("Couldn't save game '{}': {:?}", savegame_url, e)));
                }
            }
            f.close();
            Ok(VVal::Bol(true))
        });
        sscg_wl_mod.set("game", o);

        let vox_painters = Rc::new(RefCell::new(vec![]));
        let vox_painters_r = vox_painters.clone();
        sscg_wl_mod.fun("new_voxel_painter", move |_e: &mut Env, _argc: usize| {
            let (painter_ref, obj) = new_voxel_painter(vox_painters_r.borrow().len());
            vox_painters_r.borrow_mut().push(painter_ref);
            Ok(obj)
        }, Some(0), Some(0), false);

        genv.borrow_mut().set_module("sscg", sscg_wl_mod);

        let tp = TreePainter::new(fh.clone());
        Self {
            tp,
            wm,
            cmd_queue,
            vox_painters,
            fonts:           fh,
            temp_stations:   vec![(1, 1), (900, 500)],
            update_stations: true,
            wlctx:           EvalContext::new(genv),
            state:           VVal::Nul,
        }
    }

    pub fn call_cb(&mut self, name: &str, args: &[VVal]) -> VVal {
        let cb =
            match self.state.get_key("callbacks")
                      .expect("Expected 'code' in STATE!")
                      .get_key(name) {
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
            !:global STATE = main:STATE;
            main:init[]")
        {
            Ok(state) => {
                self.state = state.clone();
                dbg!("SET STATE INIT!");
            },
            Err(e) => { godot_print!("main.wl error: {:?}", e); }
        }
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
