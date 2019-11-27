use gdnative::Color;
use gdnative::Variant;
use gdnative::VariantType;
use wlambda::VVal;

pub fn variant2vval(v: &Variant) -> VVal {
    match v.get_type() {
        VariantType::Nil         => VVal::Nul,
        VariantType::Bool        => VVal::Bol(v.to_bool()),
        VariantType::I64         => VVal::Int(v.to_i64()),
        VariantType::F64         => VVal::Flt(v.to_f64()),
        VariantType::GodotString => VVal::new_str_mv(v.to_string()),
        VariantType::Dictionary => {
            let map = VVal::map();
            let dict = v.to_dictionary();
            let keys = dict.keys();
            for i in 0..keys.len() {
                let val = dict.get_ref(keys.get_ref(i));
                map.set_map_key(
                    keys.get_ref(i).to_string(),
                    variant2vval(val));
            }
            map
        },
        VariantType::VariantArray => {
            let lst = VVal::vec();
            let arr = v.to_array();
            for i in 0..arr.len() {
                lst.push(variant2vval(arr.get_ref(i)));
            }
            lst
        },
        _ => VVal::new_str_mv(v.to_string()),
    }
}

pub fn vval2variant(v: &VVal) -> Variant {
    match v {
        VVal::Nul => Variant::new(),
        VVal::Bol(b) => Variant::from_bool(*b),
        VVal::Int(i) => Variant::from_i64(*i),
        VVal::Flt(i) => Variant::from_f64(*i),
        VVal::Lst(_) => {
            let mut arr = gdnative::VariantArray::new();
            for i in v.iter() {
                arr.push(&vval2variant(&i));
            }
            Variant::from_array(&arr)
        },
        VVal::Map(_) => {
            let mut dict = gdnative::Dictionary::new();
            for kv in v.iter() {
                dict.set(
                    &Variant::from_str(kv.v_s_raw(0)),
                    &vval2variant(&kv.v_(1)));
            }
            Variant::from_dictionary(&dict)
        },
        _ => Variant::from_str(v.s_raw()),
    }
}

pub fn c2c(c: (u8, u8, u8, u8)) -> Color {
    Color::rgba(
        c.0 as f32 / 255.0,
        c.1 as f32 / 255.0,
        c.2 as f32 / 255.0,
        c.3 as f32 / 255.0)
}

