use crate::state::SSCG;
use gdnative::*;
use euclid::{vec2, vec3};
use wlambda::VVal;
use std::rc::Rc;
use sscg::tree_painter::{DrawCmd, TreePainter};
#[macro_use]
use crate::state::*;
use crate::util::{variant2vval, vval2variant, c2c};

#[derive(NativeClass)]
#[inherit(gdnative::Spatial)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct SystemMap {
    tmpl_station:  Option<PackedScene>,
    tmpl_asteroid: Option<PackedScene>,
    time_tick_sum: f64,
}

// XXX: We assume that PackedScene is thread safe.
//      And that the SystemMap Scene is always loaded!
unsafe impl Send for SystemMap { }

#[methods]
impl SystemMap {
    fn _init(owner: Spatial) -> Self {
        let main_font_resource =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://fonts/main_font_normal.tres"),
                GodotString::from_str("DynamicFont"),
                false);
        let main_font : DynamicFont =
            main_font_resource
                .and_then(|font_res| font_res.cast::<DynamicFont>())
                .unwrap();
        let mut sscg =
            SSCGState::new(Rc::new(FontHolder {
                main_font
            }));

        let mut global_lock = SSCG.lock().expect("Getting lock to SSCG");
        *global_lock = Some(sscg);

        dbg!("DONE INIT");

        global_lock.as_mut().unwrap().setup_wlambda();

        Self {
            tmpl_station:  None,
            tmpl_asteroid: None,
            time_tick_sum: 0.0,
        }
    }

    #[export]
    fn on_ship_arrived(&mut self, mut owner: Spatial, too_fast: bool, system: i64, entity: i64) {
        lock_sscg!(sscg);
        sscg.call_cb(
            "on_arrived", 
            &vec![VVal::Bol(too_fast),
                  VVal::Int(system),
                  VVal::Int(entity)]);
    }

    #[export]
    fn _ready(&mut self, mut owner: Spatial) {
        dbg!("READY SystemMap");
//        let mut f = File::new();
//        f.open(GodotString::from_str("res://test.txt"), 1)
//         .expect("test.txt to be there!") ;
//        let txt = f.get_as_text().to_string();
//        println!("LAODED: {}", txt);

        godot_print!("Scene Map Instanciated!");
        let scene = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://scenes/entities/Station Selector.tscn"),
            GodotString::from_str("PackedScene"),
            false,
        ).and_then(|s| s.cast::<PackedScene>())
         .expect("Expected station scene and it being a PackedScene!");
        self.tmpl_station = Some(scene);

        let scene = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://scenes/entities/Asteroid_1.tscn"),
            GodotString::from_str("PackedScene"),
            false,
        ).and_then(|s| s.cast::<PackedScene>())
         .expect("Expected asteroid scene and it being a PackedScene!");
        self.tmpl_asteroid = Some(scene);

        dbg!("READY");

        lock_sscg!(sscg);
        sscg.call_cb("on_ready", &vec![]);
    }

    #[export]
    fn _process(&mut self, mut owner: Spatial, delta: f64) {
        lock_sscg!(sscg);

        let vvship = sscg.state.get_key("ship").unwrap_or(VVal::Nul);

        let mut ship = unsafe {
            let mut ship = owner.get_node(NodePath::from_str("ship"))
                 .expect("Find 'ship' node")
                 .cast::<Spatial>()
                 .unwrap();
            ship.set(
                GodotString::from_str("no_fuel"),
                Variant::from_bool(vvship.v_ik("fuel") <= 0));
            ship.set(
                GodotString::from_str("docked"),
                Variant::from_bool(vvship.v_ik("docked") > 0));
            ship
        };

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
                        let v = vval2variant(&cmd.v_(1));
                        unsafe {
                            ship.call(GodotString::from_str("sscg_load"),
                                      &vec![v]) };
                    },
                    _ => {
                        godot_print!("Unknown WLambda->Godot Command: {}", cmd_str);
                    },
                }
            }

            self.time_tick_sum -= 0.25;
            let vgodot_state = VVal::map();
            vgodot_state.set_map_key(
                String::from("engine_on_secs"),
                VVal::Int(
                    unsafe {
                    ship.get(GodotString::from_str("engine_on_secs")) 
                        .to_i64() }));
            vgodot_state.set_map_key(
                String::from("speed"),
                VVal::Flt(
                    unsafe {
                    ship.get(GodotString::from_str("speed")) 
                        .to_f64() }));
            sscg.call_cb("on_tick", &vec![vgodot_state]);
        }

        let mut entities = unsafe {
            owner.get_node(NodePath::from_str("entities"))
                 .expect("Find 'entities' node")
                 .cast::<Spatial>()
                 .unwrap()
        };

        unsafe {
            for i in 0..entities.get_child_count() {
                let mut ent = entities.get_child(i).unwrap();
                if ent.get(GodotString::from_str("selected")).to_bool() {
                    ent.set(GodotString::from_str("selected"), Variant::from_i64(0));
                    println!("GOT SELECTION: {}", i);
                }
            }
        }

        if !sscg.update_stations { return; }

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
                let mut ins =
                    match &vis[..] {
                        "station" =>
                            self.tmpl_station.as_ref().unwrap()
                                .instance(0).unwrap()
                                .cast::<Spatial>()
                                .expect("Station must be a Spatial"),
                        _ | "asteroid_1" =>
                            self.tmpl_asteroid.as_ref().unwrap()
                                .instance(0).unwrap()
                                .cast::<Spatial>()
                                .expect("Station must be a Spatial"),
                    };
                let v = vec3(
                    -1000.0 + (x as f32 * 2000.0) / 10000.0,
                    1.0,
                    -1000.0 + (y as f32 * 2000.0) / 10000.0);
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
                entities.add_child(Some(ins.to_node()), false);
            }
            i += 1;
        });

        sscg.update_stations = false;
        dbg!("UPD STATE");
    }
}
