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
