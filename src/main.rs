//! A Rust cargo subcommand that will create initial code for the protocols crate. 
//! 
//! ## Installation
//! cargo install create-protocols-plugin
//! 
//! ## Usage
//! cargo create-protocols-plugin <Crate Name> <Protocol name>
//! 
//! # What does it do?
//! This crate will create you a crate with default functions to get started using the protocols library.
//! 
//! # Future Work
//! Build a library based on a Cap'n Proto schema.
//! Build a library based on flatbuffer schema.
//! 

extern crate failure;
extern crate clap;

pub mod utils;
pub mod createcrate;
pub mod addprotocol;

use failure::{Error, format_err};
use clap::{Arg, App, SubCommand, ArgMatches};

fn main() -> Result<(), Error> {
    let root_matches = handle_application_input();
    let matches = root_matches.subcommand_matches("create-protocols-plugin").ok_or(format_err!("No subcommand found!"))?;
    let crate_name = matches.value_of("cratename").ok_or(format_err!("No crate name found!"))?;
    let protocol_name = matches.value_of("protocolname").ok_or(format_err!("No protocol file found!"))?;

    println!("Verifying crate...");
    verify_or_create_existence_of_crate(crate_name)?;

    println!("Verifying protocols...");
    verify_or_create_existence_of_protocol(crate_name, protocol_name)?;

    Ok(())
}

fn handle_application_input() -> ArgMatches<'static> {
    App::new("Create Protocol Plugin Cargo Subcommand")
        .version("1.0")
        .bin_name("cargo")
        .author("James Prince <james.h.prince@gmail.com>")
        .about("Creates a default dynamic library compatible with the protocols crate.")
        .subcommand(SubCommand::with_name("create-protocols-plugin")
            .arg(Arg::with_name("cratename")
                .value_name("Crate Name")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("protocolname")
                .value_name("Protocol Name")
                .takes_value(true)
                .required(true)))
        .get_matches()
}

fn verify_or_create_existence_of_crate(crate_name: &str) -> Result<(), Error> {
    use createcrate::ProtocolBufferCrate;
    use std::path::PathBuf;

    let crate_path = PathBuf::from(format!("./{}", crate_name));
    if !crate_path.is_dir() {
        let mut crate_builder = ProtocolBufferCrate::new(crate_name);
        crate_builder.create()?;
    }
    Ok(())
}

fn verify_or_create_existence_of_protocol(crate_name: &str, protocol_name: &str) -> Result<(), Error> {
    use addprotocol::ProtocolBufferSchema;

    let mut protocol_builder = ProtocolBufferSchema::new(crate_name, protocol_name);
    if !protocol_builder.protocol_filepath().exists() {
        protocol_builder.create()?;
    }
    Ok(())
}