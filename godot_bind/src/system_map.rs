use crate::state::SSCG;
use gdnative::prelude::*;
use gdnative::api::*;
use euclid::{vec3};
use wlambda::VVal;
use std::rc::Rc;
use crate::state::*;
use crate::util::{variant2vval, vval2variant};

#[derive(NativeClass)]
#[inherit(Spatial)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct SystemMap {
    templates:     std::collections::HashMap<String, Ref<PackedScene, Shared>>,
    time_tick_sum: f64,
}

// XXX: We assume that PackedScene is thread safe.
//      And that the SystemMap Scene is always loaded!
unsafe impl Send for SystemMap { }

#[methods]
impl SystemMap {
    fn new(_owner: &Spatial) -> Self {
        let main_font_resource =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://fonts/main_font_normal.tres"),
                GodotString::from_str("DynamicFont"),
                false);
        let small_font_resource =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://fonts/main_font_small.tres"),
                GodotString::from_str("DynamicFont"),
                false);
        let main_font : Ref<DynamicFont, Shared> =
            main_font_resource
                .and_then(|font_res| font_res.cast::<DynamicFont>())
                .unwrap();
        let small_font : Ref<DynamicFont, Shared> =
            small_font_resource
                .and_then(|font_res| font_res.cast::<DynamicFont>())
                .unwrap();
        let sscg =
            SSCGState::new(Rc::new(FontHolder {
                main_font,
                small_font,
            }));

        let mut global_lock = SSCG.lock().expect("Getting lock to SSCG");
        *global_lock = Some(sscg);

        dbg!("DONE INIT");

        global_lock.as_mut().unwrap().setup_wlambda();

        Self {
            templates:     std::collections::HashMap::new(),
            time_tick_sum: 0.0,
        }
    }

    #[export]
    fn wl_cb(&mut self, _owner: &Spatial, cb_name: String, v: Variant) -> Variant {
        lock_sscg!(sscg);
        let vv = variant2vval(&v);
        let mut args = vec![];
        if vv.is_vec() {
            for (v, _) in vv.iter() {
                args.push(v);
            }
        } else {
            args.push(vv);
        }
        vval2variant(&sscg.call_cb(&cb_name, &args))
    }

    fn load_scene(&mut self, key: &str, path: &str) {
        let scene = ResourceLoader::godot_singleton().load(
            GodotString::from_str(path),
            GodotString::from_str("PackedScene"),
            false,
        ).and_then(|s| s.cast::<PackedScene>())
         .expect(
            &format!("Expected PackedScene at '{}' (for type={})", path, key));
        println!("Loaded scene {}={}", key, path);
        self.templates.insert(key.to_string(), scene);
    }

    #[export]
    fn _ready(&mut self, mut _owner: &Spatial) {
        dbg!("READY SystemMap");
//        let mut f = File::new();
//        f.open(GodotString::from_str("res://test.txt"), 1)
//         .expect("test.txt to be there!") ;
//        let txt = f.get_as_text().to_string();
//        println!("LAODED: {}", txt);

        godot_print!("Scene Map Instanciated!");
        self.load_scene("station",    "res://scenes/entities/Station Selector.tscn");
        self.load_scene("asteroid_1", "res://scenes/entities/Asteroid_1.tscn");
        self.load_scene("stargate",   "res://scenes/entities/Stargate.tscn");
        self.load_scene("structure",  "res://scenes/entities/VoxelStructure.tscn");

        lock_sscg!(sscg);
        sscg.call_cb("on_ready", &vec![]);
    }

    fn update_stations(&mut self, sscg: &mut SSCGState, entities: TRef<Spatial, Shared>) {
        if !sscg.update_stations { return; }

        let vvship = sscg.state.get_key("ship").unwrap_or(VVal::None);
        let sys_id = vvship.v_ik("system_id");
        let sys    = sscg.state.v_k("systems").v_(sys_id as usize);
        let types  = sscg.state.v_k("entity_types");

        println!("DRAWING SYSTEM: {}", sys.v_sk("name"));

        let mut i = 0;
        sys.v_k("entities").for_each(|ent: &VVal| {
            let vis = types.v_k(&ent.v_s_rawk("t")).v_s_rawk("visual");
            let pos = ent.v_k("pos");
            let x   = pos.v_i(0);
            let y   = pos.v_i(1);
            println!("ENT! {} {},{}", ent.s(), x, y);
            unsafe {
                let tmpl = unsafe {
                    self.templates.get(&vis)
                        .expect("Entry for this entity")
                        .assume_safe()
                };
                let ins = unsafe {
                    tmpl
                    .instance(0)
                    .expect("Instance in Spatial")
                    .assume_safe()
                };
                let ins =
                    ins.cast::<Spatial>()
                       .expect("Scene must be a Spatial");
                let v = vec3(x as f32, 1.0, y as f32);
                ins.set(
                    GodotString::from_str("label_name"),
                    Variant::from_str(ent.v_s_rawk("name")));
                ins.set(
                    GodotString::from_str("system_id"),
                    Variant::from_i64(sys_id));
                ins.set(
                    GodotString::from_str("entity_id"),
                    Variant::from_i64(i));
                ins.translate(v);
                entities.add_child(ins, false);
            }
            i += 1;
        });

        sscg.update_stations = false;
    }

    fn handle_commands(&mut self, sscg: &mut SSCGState, owner: &Spatial, delta: f64) {
        let vvship = sscg.state.v_k("ship").v_k("_data");

        let ship = unsafe {
            owner.get_node(NodePath::from_str("ship"))
            .expect("Find 'ship' node")
            .assume_safe()
        };
        let ship = ship.cast::<Spatial>().unwrap();
        ship.set(
            GodotString::from_str("no_fuel"),
            Variant::from_bool(vvship.v_ik("fuel") <= 0));
        ship.set(
            GodotString::from_str("docked"),
            Variant::from_bool(vvship.v_ik("docked") > 0));

        self.time_tick_sum += delta;
        while self.time_tick_sum > 0.25 {
            let cmds = std::mem::replace(&mut *sscg.cmd_queue.borrow_mut(), std::vec::Vec::new());
            for cmd in cmds {
                let cmd_str = cmd.v_s_raw(0);
                match &cmd_str[..] {
                    "save_state" => {
                        let v = unsafe {
                            ship.call(GodotString::from_str("sscg_save"),
                                      &vec![]) };
                        let vv = variant2vval(&v);
                        sscg.call_cb("on_saved_godot_state", &vec![vv]);
                    },
                    "load_state" => {
                        println!(">>>>> CMD: {}", cmd.s());
                        let v = vval2variant(&cmd.v_(1));
                        unsafe {
                            ship.call(GodotString::from_str("sscg_load"),
                                      &vec![v]) };
                    },
                    "gd_call" => {
                        let path   = cmd.v_s_raw(1);
                        let method = cmd.v_s_raw(2);
                        let node = unsafe {
                            owner.get_node(NodePath::from_str(&path)).map(|n| n.assume_safe())
                        };

                        match node {
                            Some(n) => {
                                let mut argv = vec![];
                                for argidx in 3..cmd.len() {
                                    argv.push(vval2variant(&cmd.v_(argidx)));
                                }
                                n.call(GodotString::from_str(&method), &argv);
                                godot_print!("CALLED {} . {}", path, method);
                            },
                            None => {
                                godot_print!(
                                    "Couldn't find godot node in gd_call: {}",
                                    path);
                            },
                        }
                    },
                    _ => {
                        godot_print!("Unknown WLambda->Godot Command: {}", cmd_str);
                    },
                }
            }

            self.time_tick_sum -= 0.25;
            let vgodot_state = VVal::map();
            vgodot_state.set_key_str(
                "engine_on_secs",
                VVal::Int(
                    unsafe {
                        ship.get(GodotString::from_str("engine_on_secs")) 
                            .to_i64() }));
            vgodot_state.set_key_str(
                "speed",
                VVal::Flt(
                    unsafe {
                    ship.get(GodotString::from_str("speed")) 
                        .to_f64() }));
            sscg.call_cb("on_tick", &vec![vgodot_state]);
        }

    }

    fn get_entities_node(&mut self, owner: &Spatial) -> Ref<Spatial, Shared> {
        unsafe {
            owner.get_node(NodePath::from_str("entities"))
                 .expect("Find 'entities' node")
                 .assume_safe()
                 .cast::<Spatial>()
                 .unwrap()
                 .claim()
        }
    }

    #[export]
    fn _process(&mut self, owner: &Spatial, delta: f64) {
        let entities = self.get_entities_node(owner);

        let load_entity_state = {
            lock_sscg!(sscg);

            self.handle_commands(sscg, owner, delta);

            let entities = unsafe { entities.assume_safe() };
            for i in 0..entities.get_child_count() {
                let ent = unsafe {
                    entities.get_child(i).unwrap().assume_safe()
                };

                if ent.get(GodotString::from_str("selected")).to_bool() {
                    ent.set(GodotString::from_str("selected"), Variant::from_i64(0));
                    println!("GOT SELECTION: {}", i);
                }
            }

            let load_entity_state = sscg.update_stations;

            self.update_stations(sscg, entities);

            load_entity_state
        };

        if load_entity_state {
            let entities = unsafe { entities.assume_safe() };
            for i in 0..entities.get_child_count() {
                let ent = unsafe {
                    entities.get_child(i).unwrap().assume_safe()
                };
                println!("LES {}", i);
                ent.call(GodotString::from_str("on_wlambda_init"), &vec![]);
            }
        }
    }
}
