extern crate protobuf;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate failure;

pub mod __PROTOCOLBUFFERNAME__;
use __PROTOCOLBUFFERNAME__::__PROTOCOLBUFFERSTRUCT__ as TargetStructure;
use failure::Error;

#[no_mangle]
pub extern fn get_name() -> String {
    return "__PROTOCOLBUFFERSTRUCT__".to_string();
}

#[no_mangle]
pub extern fn handle(data: &[u8]) -> Result<(), Error> {
    let string: String = data.iter().map(|u: &u8| *u as char).collect();
    println!("Handling: {:?}", data);
    let structure: TargetStructure = serde_json::from_str(&string)?;
    println!("Received message: {:?}", structure);
    Ok(())
}

#[no_mangle]
pub extern fn generate_message(template_name: &str) -> Result<String, Error> {
    // For now, just generate a default message
    let structure = TargetStructure::new();
    Ok(serde_json::to_string(&structure)?)
}

#[no_mangle]
pub extern fn get_hash() -> String{
    return include_str!("../hash.txt").to_string();
}
