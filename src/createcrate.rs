extern crate failure;

use std::path::PathBuf;

use failure::{Error, format_err};

fn append_to_file(new_file: &PathBuf, contents: &str) -> Result<(), Error> {
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

fn overwrite_file(new_file: &PathBuf, contents: &str) -> Result<(), Error> {
    use std::fs::OpenOptions;
    use std::io::Write;
    println!("Writing to: {:?}", new_file);
    let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(false)
                    .open(new_file)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn sleep_ms(ms: u64) {
    use std::{thread, time};
    let time = time::Duration::from_millis(ms);
    thread::sleep(time);
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug)]
pub struct ProtocolBufferCrate {
    crate_name: String,
    schema_path: PathBuf,
    basename: String,
    filename: String,
}

impl ProtocolBufferCrate {
    pub fn new(crate_name: &str, schema_path: &str) -> Self {
        ProtocolBufferCrate { crate_name: crate_name.to_string(), 
                              schema_path: PathBuf::from(schema_path), 
                              basename: String::new(),
                              filename: String::new(),
                             }
    }

    pub fn create(&mut self) -> Result<(), Error> {
        self.parse_and_verify_schema_path()?;
        println!("{:?}", self);

        self.build_crate()?;
        sleep_ms(1000);

        self.create_schema_directory()?;
        self.copy_schema_to_schema_directory()?;
        self.create_lib_rs_from_template()?;
        self.create_build_rs_from_template()?;
        self.append_template_dependencies_to_cargotoml()?;

        Ok(())
    }
    
    fn struct_name(&self) -> String {
        uppercase_first_letter(&self.basename)
    }

    fn parse_and_verify_schema_path(&mut self) -> Result<(), Error> {
        let filename = self.schema_path.file_name().ok_or(format_err!("Cannot parse filename!"))?;
        let filename = filename.to_str().ok_or(format_err!("Cannot convert osStr to str."))?;
        let basename = self.schema_path.file_stem().ok_or(format_err!("Cannot find file!"))?;
        let basename = basename.to_str().ok_or(format_err!("Cannot convert osStr to str."))?;
        
        if !self.schema_path.exists() {
            return Err(format_err!("Warning! Schema {:?} does not exist!", &self.schema_path));
        }

        self.basename = basename.to_string();
        self.filename = filename.to_string();

        Ok(())
    }

    fn append_template_dependencies_to_cargotoml(&self) -> Result<(), Error> {
        let file_content = include_str!("../templates/dependencies.txt");
        let mut new_file = self.crate_directory();
        new_file.push("Cargo.toml");
        println!("Writing from dependencies.txt template to {:?}", new_file);
        append_to_file(&new_file, file_content)?;
        Ok(())
    }

    fn build_crate(&self) -> Result<(), Error> {
        use std::process::Command;
        use std::str;

        println!("Building crate...");
        let output = Command::new("cargo")
                            .args(&["new", &self.crate_name, "--lib"])
                            .output()?;

        if output.stdout.len() > 0 {
            println!("{}", str::from_utf8(&output.stdout)?); }
        if output.stderr.len() > 0 {
            println!("{}", str::from_utf8(&output.stderr)?); }
        Ok(())
    }   

    fn crate_directory(&self) -> PathBuf { 
        let mut dir = PathBuf::from("./");
        dir.push(&self.crate_name);
        dir
    }

    fn schema_directory(&self) -> PathBuf { 
        let mut dir = self.crate_directory();
        dir.push("schema");
        dir
    }

    fn create_schema_directory(&self) -> Result<(), Error> {
        use std::fs;
        let dir = self.schema_directory();
        println!("Creating schema directory...");
        fs::create_dir_all(dir)?;
        Ok(())
    }

    fn copy_schema_to_schema_directory(&self) -> Result<(), Error> {
        use std::fs;
        // Example: Copy something.proto to ./<crate>/schema/something.proto
        let from = &self.schema_path;
        let mut to = self.schema_directory();
        to.push(&self.filename);
        println!("Copying {:?} to {:?}...", from, to);
        fs::copy(from, to)?;
        Ok(())
    }

    fn get_replace_strings_in_template(&self, data: &str) -> String {
        let data = data.replace("__PROTOCOLBUFFERNAME__", &self.basename);
        let data = data.replace("__PROTOCOLBUFFERSTRUCT__", &self.struct_name());
        data
    }

    fn create_lib_rs_from_template(&self) -> Result<(), Error> {
        println!("Creating lib.rs from template...");
        let file_content = include_str!("../templates/lib.rs");
        let file_content = self.get_replace_strings_in_template(file_content);
        
        let mut new_filename = self.crate_directory();
        new_filename.push("src");
        new_filename.push("lib.rs");
        overwrite_file(&new_filename, &file_content)?;
        Ok(())
    }

    fn create_build_rs_from_template(&self) -> Result<(), Error> {
        println!("Creating build.rs from template...");
        let file_content = include_str!("../templates/build.rs");
        let file_content = self.get_replace_strings_in_template(file_content);
        
        let mut new_filename = self.crate_directory();
        new_filename.push("build.rs");
        overwrite_file(&new_filename, &file_content)?;
        Ok(())
    }
}
