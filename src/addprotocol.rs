extern crate failure;

use std::path::PathBuf;

use failure::Error;
use utils;

#[derive(Debug)]
pub struct ProtocolBufferSchema {
    crate_name: String,
    protocol_name: String,
}

impl ProtocolBufferSchema {
    pub fn new(crate_name: &str, protocol_name: &str) -> Self {
        ProtocolBufferSchema { crate_name: crate_name.to_string(), 
                               protocol_name: protocol_name.to_string(),
                             }
    }

    pub fn create(&mut self) -> Result<(), Error> {
        self.create_schema()?;
        self.create_schema_interface_rs_from_template()?;
        self.insert_protocol_info_into_lib_rs()?;
        Ok(())
    }

    fn insert_protocol_info_into_lib_rs(&self) -> Result<(), Error> {
        use std::fs;
        let lib_rs = fs::read_to_string(self.lib_rs_filepath())?;
        let lib_rs = lib_rs.replace("// __PUBMODPROTOCOLS__", &self.get_pub_mod_protocol_replacement_string());
        let lib_rs = lib_rs.replace("// __REGISTERINTERFACES__", &self.get_interface_string());
        fs::write(self.lib_rs_filepath(), lib_rs)?;

        Ok(())
    }

    fn get_pub_mod_protocol_replacement_string(&self) -> String {
        format!("pub mod {IFACE};\npub mod {IFACE}_interface;\n// __PUBMODPROTOCOLS__", IFACE = self.protocol_name)
    }

    fn get_interface_string(&self) -> String { 
        // m.insert(test_interface::TestInterface::get_schema_url(), Box::new(test_interface::TestInterface{}));
        let interface_str = format!("{}_interface::{}Interface", self.protocol_name, utils::uppercase_first_letter(&self.protocol_name));
        let interface_str = format!("m.insert({IFACE}::get_schema_url(), Box::new({IFACE}{{}}));\n\t\t// __REGISTERINTERFACES__", IFACE = interface_str);
        interface_str
    }

    fn struct_name(&self) -> String {
        utils::uppercase_first_letter(&self.protocol_name)
    }

    fn protocol_filename(&self) -> PathBuf {
        PathBuf::from(format!("{}.proto", self.protocol_name))
    }

    fn crate_directory(&self) -> PathBuf { 
        let mut dir = PathBuf::from("./");
        dir.push(&self.crate_name);
        dir
    }

    pub fn protocol_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push("schema");
        path.push(self.protocol_filename());
        path
    }

    fn protocol_interface_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push(format!("src/{}_interface.rs", self.protocol_name));
        path
    }

    fn lib_rs_filepath(&self) -> PathBuf {
        let mut path = self.crate_directory();
        path.push("src/lib.rs");
        path
    }

    fn create_schema(&mut self) -> Result<(), Error> {
        use std::fs;
        let schema_data = include_str!("../templates/default.proto").to_string();
        let schema_data = schema_data.replace("__SCHEMANAME__", &self.struct_name());

        fs::write(&self.protocol_filepath(), &schema_data)?;

        Ok(())
    }

     fn create_schema_interface_rs_from_template(&self) -> Result<(), Error> {
        use std::fs;
        let schema_data = include_str!("../templates/schema_interface.rs").to_string();
        let schema_data = schema_data.replace("__PROTOCOL_NAME__", &self.protocol_name);
        let schema_data = schema_data.replace("__PROTOCOL_STRUCT_NAME__", &self.struct_name());

        fs::write(&self.protocol_interface_filepath(), &schema_data)?;

        Ok(())
    }
}
