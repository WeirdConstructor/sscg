use crate::state::SSCG;
use gdnative::*;
use euclid::vec3;
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
        let mut f = File::new();
        f.open(GodotString::from_str("res://test.txt"), 1)
         .expect("test.txt to be there!");
        let txt = f.get_as_text().to_string();
        println!("LAODED: {}", txt);

        let mut sscg_lck = SSCG.lock().unwrap();
        let sscg = sscg_lck.as_mut().unwrap();
        sscg.setup_wlambda();

        godot_print!("Scene Map Instanciated!");
        let scene = ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://scenes/entities/Station Selector.tscn"),
            GodotString::from_str("PackedScene"),
            false,
        ).and_then(|s| s.cast::<PackedScene>())
         .expect("Expected system scene and it being a PackedScene!");
        self.tmpl_station = Some(scene);
    }

    #[export]
    fn _process(&mut self, mut owner: Spatial, delta: f64) {
        let mut sscg_lck = SSCG.lock().unwrap();
        let sscg = sscg_lck.as_mut().unwrap();
        if !sscg.update_stations { return; }

        for s in sscg.temp_stations.iter() {
            unsafe {
                let mut ins =
                    self.tmpl_station.as_ref().unwrap()
                        .instance(0).unwrap()
                        .cast::<Spatial>()
                        .expect("Station must be a Spatial");
                ins.translate(vec3(
                    -8.0 + (s.0 as f32 * 16.0) / 1000.0,
                    1.0,
                    -8.0 + (s.1 as f32 * 16.0) / 1000.0));
                owner.add_child(Some(ins.to_node()), false);
                println!("ADDDED STATION {:?}", s);
            }
        }

        sscg.update_stations = false;
    }
}
