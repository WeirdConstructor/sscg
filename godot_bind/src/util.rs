use gdnative::core_types::Color;
use gdnative::core_types::Variant;
use gdnative::core_types::VariantType;
use gdnative::core_types::VariantArray;
use gdnative::core_types::Dictionary;
use wlambda::VVal;
use wlambda::util::{SplitMix64, u64_to_open01};

pub fn variant2vval(v: &Variant) -> VVal {
    match v.get_type() {
        VariantType::Nil         => VVal::None,
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
                map.set_key_str(
                    &keys.get_ref(i).to_string(),
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
        VVal::None   => Variant::new(),
        VVal::Bol(b) => Variant::from_bool(*b),
        VVal::Int(i) => Variant::from_i64(*i),
        VVal::Flt(i) => Variant::from_f64(*i),
        VVal::Lst(_) => {
            let mut arr = VariantArray::new();
            for (i, _) in v.iter() {
                arr.push(&vval2variant(&i));
            }
            Variant::from_array(&arr.into_shared())
        },
        VVal::Map(_) => {
            let mut dict = Dictionary::new();
            for (v, k) in v.iter() {
                dict.insert(
                    &Variant::from_str(k.unwrap().s_raw()),
                    &vval2variant(&v));
            }
            Variant::from_dictionary(&dict.into_shared())
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

pub struct WorkerPool<J,R> where J: Send, R: Send
{
    next_worker_idx: usize,
    workers: std::vec::Vec<WorkerThreadRef<J>>,
    result_rx: std::sync::mpsc::Receiver<R>,
    queued_job_count: usize,
}

impl<J,R> WorkerPool<J,R>
    where J: Send + 'static, R: Send + 'static
{
    pub fn new<F>(f: F, worker_count: usize) -> Self
        where F: Fn(J) -> R,
              F: Send + 'static,
              F: Clone,
    {
        let (result_tx, result_rx) = std::sync::mpsc::channel();
        let mut workers = vec![];

        for i in 0..worker_count {
            let name = format!("T[{}]", i);
            workers.push(
                WorkerThreadRef::new(
                    name,
                    f.clone(), result_tx.clone()));
        }

        Self {
            next_worker_idx: 0,
            result_rx,
            workers,
            queued_job_count: 0,
        }
    }

    pub fn send(&mut self, j: J) {
        self.workers[self.next_worker_idx].job_tx.send(j).unwrap();
        self.queued_job_count += 1;
        self.next_worker_idx += 1;
        if self.next_worker_idx >= self.workers.len() {
            self.next_worker_idx = 0;
        }
    }

    pub fn queued_job_count(&self) -> usize { self.queued_job_count }

    pub fn get_result_blocking(&mut self) -> Option<R> {
        match self.result_rx.recv() {
            Ok(v) => {
                self.queued_job_count -= 1;
                Some(v)
            },
            _ => None,
        }
    }

    pub fn get_result(&mut self) -> Option<R> {
        match self.result_rx.try_recv() {
            Ok(v) => {
                self.queued_job_count -= 1;
                Some(v)
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct WorkerThreadRef<J>
    where J: Send
{
    job_tx: std::sync::mpsc::Sender<J>,
}

impl<J> WorkerThreadRef<J>
    where J: Send,
{
    pub fn new<F,R>(name: String, f: F, result_tx: std::sync::mpsc::Sender<R>) -> Self
        where F: Fn(J) -> R,
              R: Send + 'static,
              J: Send + 'static,
              F: Send + 'static
    {
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::Builder::new().name(name).spawn(move ||{
            loop {
                let job = match rx.recv() {
                    Ok(j) => j,
                    Err(e) => {
                        eprintln!("Stopping worker thread, sender closed: {}", e);
                        return;
                    },
                };
                let res = f(job);
                match result_tx.send(res) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("Stopping worker thread, receiver closed: {}", e);
                        return;
                    },
                }
            }
        });

        Self {
            job_tx: tx,
        }
    }
}

pub fn write_file_safely(filename: &str, s: &str) -> std::io::Result<()> {
    use std::io::Write;
    let tmpfile = format!("{}~", filename);
    let mut file = std::fs::File::create(tmpfile.clone())?;
    file.write_all(s.as_bytes())?;
    std::fs::rename(tmpfile, filename)?;
    Ok(())
}

pub fn read_vval_json_file(filename: &str) -> VVal {
    use std::io::Read;
    match std::fs::File::open(filename) {
        Ok(mut file) => {
            let mut c = String::new();
            match file.read_to_string(&mut c) {
                Ok(_) => {
                    match VVal::from_json(&c) {
                        Ok(v) => {
                            v
                        },
                        Err(e) => {
                            println!("SAVE DESERIALIZE ERROR: {}", e);
                            VVal::None
                        },
                    }
                },
                Err(e) => {
                    println!("SAVE READ ERROR: {}", e);
                    VVal::None
                }
            }
        },
        Err(e) => {
            println!("SAVE OPEN ERROR: {}", e);
            VVal::None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sampled3DNoise {
    data: std::vec::Vec<f64>,
    size: usize,
}

pub fn smoothstep_f64(a: f64, b: f64, x: f64) -> f64
{
    let x = x.max(0.0).min(1.0);
    let x = x * x * (3.0 - 2.0 * x);
    let r = a * (1.0 - x) + b * x;
    r
}

impl Sampled3DNoise {
    pub fn new(size: usize, seed: i64) -> Self {
        let size = size + 1;
        let mut data = vec![];
        data.resize(size * size * size, 0.0);
        let mut sm = SplitMix64::new_from_i64(seed);
        for i in 0..(size * size * size) {
            data[i] = u64_to_open01(sm.next_u64());
        }
        Self { size, data }
    }

    pub fn at_fbm(&self, x: f64, y: f64, z: f64,
                  octaves: usize,
                  lacunarity: f64,
                  gain: f64) -> f64
    {
        let mut freq = 1.0;
        let mut amp = 0.5;
        let mut res = 0.0;
        let mut amp_cor = 0.0;
        for _o in 0..octaves {
            let v = self.at(
                x * freq,
                y * freq,
                z * freq);

//            println!("AT FBM: x={}, y={}, z={}, freq={}, ns={}, v={}",
//                     x, y, z, freq, noise_size, v);

            res     += amp * v;
            amp_cor += amp;

            freq *= lacunarity;
            amp  *= gain;
        }
//        println!("FBMOUT: res={}, ac={} => {}", res, amp_cor, res / amp_cor);
        res / amp_cor
    }

    pub fn at(&self, x: f64, y: f64, z: f64) -> f64 {
        let xf = x.fract();
        let yf = y.fract();
        let zf = z.fract();
        let x = x.floor() as usize % (self.size - 1);
        let y = y.floor() as usize % (self.size - 1);
        let z = z.floor() as usize % (self.size - 1);

        let x_stride : usize = 1;
        let y_stride : usize = self.size;
        let z_stride : usize = y_stride * y_stride;

        let base : usize = z * z_stride       + y * y_stride       + x;

        let c_0 = self.data[base];
        let c_x = self.data[base + x_stride];

        let c_y = self.data[base + y_stride];
        let c_xy = self.data[base + y_stride + x_stride];

        let c_z = self.data[base + z_stride];
        let c_xz = self.data[base + z_stride + x_stride];

        let c_yz = self.data[base + z_stride + y_stride];
        let c_xyz = self.data[base + z_stride + y_stride + x_stride];

        let e_x = smoothstep_f64(c_0, c_x, xf);
        let e_y = smoothstep_f64(c_y, c_xy, xf);
        let e_z = smoothstep_f64(c_z, c_xz, xf);
        let e_all = smoothstep_f64(c_yz, c_xyz, xf);

        let z0 = smoothstep_f64(e_x, e_y, yf);
        let z1 = smoothstep_f64(e_z, e_all, yf);

        smoothstep_f64(z0, z1, zf)
    }
}
