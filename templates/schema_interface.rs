extern crate protobuf;
extern crate serde_json;
extern crate serde;
extern crate failure;
extern crate protocols;

use __PROTOCOL_NAME__::__PROTOCOL_STRUCT_NAME__;
use self::failure::{Error};
use self::protocols::pluginhandler::{MessageInfo, SubLibrary};

pub struct __PROTOCOL_STRUCT_NAME__Interface;

impl SubLibrary for __PROTOCOL_STRUCT_NAME__Interface {
    fn get_name(&self) -> String {
        return "__PROTOCOL_STRUCT_NAME__".to_string();
    }

    fn handle(&self, info: MessageInfo) -> Result<Vec<MessageInfo>, Error> {
        let string: String = info.data.iter().map(|u: &u8| *u as char).collect();
        println!("Handling: {}", string);
        let structure: __PROTOCOL_STRUCT_NAME__ = serde_json::from_str(&string)?;
        println!("Received message: {:?}", structure);

        Ok(Vec::new())
    }

    fn get_schema_url() -> String {
        return include_str!("../schema_urls/__PROTOCOL_NAME__.txt").to_string();
    }
}
