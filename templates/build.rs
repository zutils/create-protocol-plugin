extern crate failure;
extern crate protocols;

use protocols::buildfunctions;
use failure::Error;
use std::path::PathBuf;

fn create_protobuf(proto_path: &PathBuf) -> Result<(), Error> {
	buildfunctions::build_rust_code_from_protobuffer(proto_path)?;
	buildfunctions::build_rust_rpc_code_from_protobuffer(proto_path)?;

	// Comment this line out if you want to handle writing to ./schema_urls/<your protocol>.txt manually.
	buildfunctions::add_file_and_write_ipfs_hash(proto_path)?;
	Ok(())
}

fn main()  {
	buildfunctions::for_all_in_dir("./schema/", |path| create_protobuf(path));
}
