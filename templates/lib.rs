#[macro_use] extern crate serde_derive;
extern crate protobuf;
extern crate serde_json;
extern crate serde;
extern crate failure;
extern crate protocols;

pub mod __PROTOCOLBUFFERNAME__;
use __PROTOCOLBUFFERNAME__::__PROTOCOLBUFFERSTRUCT__;
use failure::Error;
use protocols::pluginhandler::MessageInfo;

#[no_mangle]
pub extern fn get_name() -> String {
    return "__PROTOCOLBUFFERSTRUCT__".to_string();
}

#[no_mangle]
pub extern fn handle(info: &MessageInfo) -> Result<Vec<MessageInfo>, Error> {
    let string: String = info.data.iter().map(|u: &u8| *u as char).collect();
    println!("Handling: {:?}", string);
    let structure: __PROTOCOLBUFFERSTRUCT__ = serde_json::from_str(&string)?;
    println!("Received message: {:?}", structure);

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

// TODO: This should be replaced with a way to query the RPC.
#[no_mangle]
pub extern fn get_non_standard_library_interface_functions() -> Vec<String> {
    let ret = Vec::new();
    //ret.push("non_standard_function");
    ret
}