use wlambda::{VVal, StackAction, GlobalEnv, EvalContext, SymbolTable};
use gdnative::godot_print;
use gdnative::{File, GodotString};
use wlambda::compiler::{GlobalEnvRef, ModuleResolver, ModuleLoadError};

/// This structure implements the ModuleResolver trait and is
/// responsible for loading modules on `!@import` for WLambda.
#[derive(Debug, Clone, Default)]
pub struct GodotModuleResolver { }

#[allow(dead_code)]
impl GodotModuleResolver {
    pub fn new() -> GodotModuleResolver {
        GodotModuleResolver { }
    }
}

impl ModuleResolver for GodotModuleResolver {
    fn resolve(&mut self, global: GlobalEnvRef, path: &[String])
        -> Result<SymbolTable, ModuleLoadError>
    {
        let genv = GlobalEnv::new_empty_default();
        genv.borrow_mut().import_modules_from(&*global.borrow());
        let mut ctx = EvalContext::new(genv);
        let pth = path.join("/");

        let mut f = File::new();
        let mod_path = format!("res://gamelib/{}.wl", pth.clone());
        godot_print!("PRINT {}", pth.clone());
        match f.open(GodotString::from_str(&mod_path), 1)
        {
            Ok(_) => {
                let txt = f.get_as_text().to_string();
                match ctx.eval_string(&txt, &(pth.clone() + ".wl")) {
                    Err(e) => Err(ModuleLoadError::ModuleEvalError(e)),
                    Ok(_v) => Ok(ctx.get_exports()),
                }
            },
            Err(e) => {
                godot_print!("Couldn't load module: '{}': {:?}", pth, e);
                Err(ModuleLoadError::NoSuchModule)
            },
        }
    }
}

