extern crate failure;

use std::path::PathBuf;

use failure::Error;

pub fn sleep_ms(ms: u64) {
    use std::{thread, time};
    let time = time::Duration::from_millis(ms);
    thread::sleep(time);
}

pub fn append_to_file(new_file: &PathBuf, contents: &str) -> Result<(), Error> {
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(new_file)?;
    println!("Writing to: {:?}", new_file);
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
