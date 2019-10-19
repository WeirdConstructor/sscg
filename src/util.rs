use wlambda::VVal;

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
                            VVal::Nul
                        },
                    }
                },
                Err(e) => {
                    println!("SAVE READ ERROR: {}", e);
                    VVal::Nul
                }
            }
        },
        Err(e) => {
            println!("SAVE OPEN ERROR: {}", e);
            VVal::Nul
        }
    }
}
