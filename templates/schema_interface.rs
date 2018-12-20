use crate::__PROTOCOL_NAME___autogen::__PROTOCOL_NAME__;
use failure::Error;
use protocols::{CommonModule, Data, ModuleInfo, VecModuleInfo, VecData, Destination, GenerateMessageInfo, VecRpcData, RpcData};
use protocols::utils::{ToDataConverter, FromDataConverter, schema_ipfs_from_str};

static SCHEMA_URL: &str = include_str!("../schema_urls/__PROTOCOL_NAME__.txt");

pub struct __PROTOCOL_STRUCT_NAME__Interface;

impl ToDataConverter for __PROTOCOL_NAME__::__PROTOCOL_STRUCT_NAME__ {}

impl CommonModule for __PROTOCOL_STRUCT_NAME__Interface {
    fn get_info(&self, _: &Destination) -> Result<VecModuleInfo, Error> {
        let mut info = ModuleInfo::new();
        info.set_name("__PROTOCOL_STRUCT_NAME__".to_string());
        info.set_schema(schema_ipfs_from_str(SCHEMA_URL));
        
        let mut ret = VecModuleInfo::new();
        ret.vec = protobuf::RepeatedField::from_vec(vec![info]);
        Ok(ret)
    }

    fn generate_message(&self, data: &GenerateMessageInfo) -> Result<Data, Error> {
        use std::str;
        let template = data.get_template();
        let _args = data.get_args();
        match template {
            _ => Err(failure::format_err!("Unrecognized template {:?}. 'Root' available.", template)),
        }
    }

    fn handle_trusted(&self, data: &Data) -> Result<VecData, Error> {
        let (schema, structure) = data.unwrap::<__PROTOCOL_NAME__::__PROTOCOL_STRUCT_NAME__>()?;
        println!("Received __PROTOCOL_STRUCT_NAME__ Message: ({:?},{:?})", schema, structure);

        let ret = VecData::new();
        Ok(ret)
    }

    fn receive_trusted_rpc(&self, _data: &RpcData) -> Result<VecRpcData, Error> {
        Err(failure::format_err!("No Trusted Rpc for {:?}", SCHEMA_URL))
    }

    fn receive_untrusted_rpc(&self, _data: &RpcData) -> Result<VecRpcData, Error> {
        Err(failure::format_err!("No Untrusted Rpc for {:?}", SCHEMA_URL))
    }
}

