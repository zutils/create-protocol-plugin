extern crate failure;

use std::path::PathBuf;

use failure::Error;
use utils;

#[derive(Debug)]
pub struct ProtocolBufferCrate {
    crate_name: String,
}

impl ProtocolBufferCrate {
    pub fn new(crate_name: &str) -> Self {
        ProtocolBufferCrate { crate_name: crate_name.to_string() }
    }

    pub fn create(&mut self) -> Result<(), Error> {
        self.build_crate()?;
        utils::sleep_ms(200);

        self.create_schema_directory()?;
        self.create_schema_urls_directory()?;
        self.create_lib_rs_from_template()?;
        self.create_build_rs_from_template()?;
        self.append_template_dependencies_to_cargotoml()?;

        Ok(())
    }

    fn append_template_dependencies_to_cargotoml(&self) -> Result<(), Error> {
        println!("Writing from dependencies.txt template to {:?}", &self.cargo_toml_filepath());
        let file_content = include_str!("../templates/dependencies.txt");
        utils::append_to_file(&self.cargo_toml_filepath(), file_content)?;
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

    fn create_schema_directory(&self) -> Result<(), Error> {
        use std::fs;
        println!("Creating schema directory...");
        fs::create_dir_all(self.schema_directory())?;
        Ok(())
    }

    fn create_schema_urls_directory(&self) -> Result<(), Error> {
        use std::fs;
        println!("Creating schema directory...");
        fs::create_dir_all(self.schema_urls_directory())?;
        Ok(())
    }

    fn create_lib_rs_from_template(&self) -> Result<(), Error> {
        use std::fs;
        println!("Creating lib.rs from template...");
        let file_content = include_str!("../templates/lib.rs");
        fs::write(&self.lib_rs_filepath(), &file_content)?;
        Ok(())
    }

    fn create_build_rs_from_template(&self) -> Result<(), Error> {
        use std::fs;
        println!("Creating build.rs from template...");
        let file_content = include_str!("../templates/build.rs");
        fs::write(&self.build_rs_filepath(), &file_content)?;
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

    fn schema_urls_directory(&self) -> PathBuf { 
        let mut dir = self.crate_directory();
        dir.push("schema_urls");
        dir
    }

    fn lib_rs_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push("src/lib.rs");
        path
    }

    fn build_rs_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push("build.rs");
        path
    }

    fn cargo_toml_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push("Cargo.toml");
        path
    }
}
