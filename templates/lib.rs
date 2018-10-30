#[macro_use] extern crate serde_derive;
extern crate protobuf;
extern crate serde_json;
extern crate serde;
extern crate failure;
extern crate protocols;

pub mod __PROTOCOLBUFFERNAME__;
use __PROTOCOLBUFFERNAME__::__PROTOCOLBUFFERSTRUCT__;
use failure::Error;
use protocols::pluginmanager::{PluginManager};

#[no_mangle]
pub extern fn get_name() -> String {
    return "__PROTOCOLBUFFERSTRUCT__".to_string();
}

#[no_mangle]
pub extern fn handle(manager: &PluginManager, data: &[u8]) -> Result<(), Error> {
    let string: String = data.iter().map(|u: &u8| *u as char).collect();
    println!("Handling: {:?}", data);
    let structure: __PROTOCOLBUFFERSTRUCT__ = serde_json::from_str(&string)?;
    println!("Received message: {:?}", structure);

    /*
    // If there are any sub-messages, you can use this code below.
        let schema_url = structure.get_schema_url(); // Use your own function here.
        let unencrypted_message = structure.get_unencrypted_message();  // Use your own function here.
        manager.handle_msg_and_submsgs(schema_url, unencrypted_message);
    */

    Ok(())
}

// This may represent a problem if root-message recurses from itself.
fn handle_submessages(manager: &PluginManager, schema_url: &str, data: &[u8]) -> Result<(), Error> {
    manager.handle_msg_and_submsgs(schema_url, data);
}

#[no_mangle]
pub extern fn generate_message(template_name: &str) -> Result<String, Error> {
    // For now, just generate a default message
    let structure = __PROTOCOLBUFFERSTRUCT__::new();
    Ok(serde_json::to_string(&structure)?)
}

#[no_mangle]
pub extern fn get_schema_url() -> String{
    return include_str!("../schema_url.txt").to_string();
}
