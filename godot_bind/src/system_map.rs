use crate::state::SSCG;
use gdnative::*;
use euclid::vec3;
use wlambda::VVal;
use std::rc::Rc;
use sscg::tree_painter::{DrawCmd, TreePainter};
use crate::state::*;
//use std::sync::{Arc, Mutex, Condvar};

#[derive(NativeClass)]
#[inherit(gdnative::Spatial)]
//#[user_data(user_data::ArcData<SystemMap>)]
pub struct SystemMap {
    tmpl_station: Option<PackedScene>,
}

// XXX: We assume that PackedScene is thread safe.
unsafe impl Send for SystemMap { }

#[methods]
impl SystemMap {
    fn _init(_owner: Spatial) -> Self { Self { tmpl_station: None } }

    #[export]
    fn _ready(&mut self, mut owner: Spatial) {
        dbg!("INIT SSCGState");
        let mut f = File::new();
        f.open(GodotString::from_str("res://test.txt"), 1)
         .expect("test.txt to be there!");
        let txt = f.get_as_text().to_string();
        println!("LAODED: {}", txt);


        let f =
            ResourceLoader::godot_singleton().load(
                GodotString::from_str("res://fonts/main_font_normal.tres"),
                GodotString::from_str("DynamicFont"),
                false);
        let df : DynamicFont = f.and_then(|f| f.cast::<DynamicFont>()).unwrap();

        let mut cmds = std::vec::Vec::new();
        cmds.push(DrawCmd::Rect { x: 0, y: 0, w: 100, h: 100, color: (255, 255, 0, 255) });
        cmds.push(DrawCmd::Rect { x: 50, y: 25, w: 100, h: 100, color: (0, 255, 0, 255) });
        cmds.push(DrawCmd::Text { x: 100, y: 100, w: 300,
            txt: String::from("FOFOFööß"),
            align: 0, color: (0, 255, 0, 255) });

        let fh: Rc<FontHolder> = Rc::new(FontHolder { main_font: df });
        let tp = TreePainter::new(fh.clone());
        let mut d = SSCG.lock().expect("Getting lock to SSCG");
        let mut sscg = SSCGState::new(fh, cmds);
        sscg.setup_wlambda();
        *d = Some(sscg);
//        let mut sscg_lck = SSCG.lock().unwrap();
//        let sscg = sscg_lck.as_mut().unwrap();

        godot_print!("Scene Map Instanciated!");
        let scene = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://scenes/entities/Station Selector.tscn"),
            GodotString::from_str("PackedScene"),
            false,
        ).and_then(|s| s.cast::<PackedScene>())
         .expect("Expected system scene and it being a PackedScene!");
        self.tmpl_station = Some(scene);
        dbg!("READY");
    }

    #[export]
    fn _process(&mut self, mut owner: Spatial, delta: f64) {
        let mut sscg_lck = SSCG.lock().unwrap();
        let sscg = sscg_lck.as_mut().unwrap();

        let vvship = sscg.state.get_key("ship").unwrap_or(VVal::Nul);
        let ship_pos = vvship.get_key("pos").unwrap_or(VVal::Nul);
        unsafe {
            let mut ship =
                owner.get_node(NodePath::from_str("ship"))
                     .expect("Find 'ship' node")
                     .cast::<Spatial>()
                     .unwrap();

            let t = vec3(
                -8.0 + ((ship_pos.v_f(0) as f32 * 16.0) / 10000.0),
                1.0,
                -8.0 + ((ship_pos.v_f(1) as f32 * 16.0) / 10000.0));
            ship.set_translation(t);

//            println!("SHIP POS: {:?}", t);
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
        let sys = sscg.state.v_k("systems").v_(sys_id as usize);

        println!("DRAWING SYSTEM: {}", sys.v_sk("name"));

        let mut i = 0;
        sys.v_k("entities").for_each(|ent: &VVal| {
            let pos = ent.v_k("pos");
            let x   = pos.v_i(0);
            let y   = pos.v_i(1);
            println!("ENT! {} {},{}", ent.s(), x, y);
            unsafe {
                let mut ins =
                    self.tmpl_station.as_ref().unwrap()
                        .instance(0).unwrap()
                        .cast::<Spatial>()
                        .expect("Station must be a Spatial");
                let v = vec3(
                    -8.0 + (x as f32 * 16.0) / 10000.0,
                    1.0,
                    -8.0 + (y as f32 * 16.0) / 10000.0);
                    println!("FO {:?}", v);
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
