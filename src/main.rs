#[macro_use] extern crate failure;
extern crate clap;

pub mod createcrate;

use createcrate::{ProtocolBufferCrate};
use failure::Error;
use clap::{Arg, App, SubCommand, ArgMatches};

fn handle_application_input() -> ArgMatches<'static> {
    App::new("Create Protocol Plugin Cargo Subcommand")
        .version("1.0")
        .bin_name("cargo")
        .author("James Prince <james.h.prince@gmail.com>")
        .about("Creates a default static or dynamic library compatible with the protocolhandler crate.")
        .subcommand(SubCommand::with_name("createprotocolplugin")
            .arg(Arg::with_name("cratename")
                .value_name("Crate Name")
                .takes_value(true)
                .required(true))
            // Commented - for now, we just create dynamic (dll) crates.
            /*.arg(Arg::with_name("static")
                .short("s")
                .long("static")
                .help(r#"Creates a static library crate. Include in [dependencies]."#)
                .takes_value(false)
                .required_unless("dynamic"))
            .arg(Arg::with_name("dynamic")
                .short("d")
                .long("dynamic")
                .help(r#"Creates a dynamic library crate. Include in the plugins directory."#)
                .required_unless("static")
                .takes_value(false))*/
            .arg(Arg::with_name("protocol")
                .short("p")
                .long("protocol")
                .value_name(".proto File")
                .help(r#"Add a .proto file."#)
                .takes_value(true)
                .required(true)))
        .get_matches()
}

fn main() -> Result<(), Error> {
    let root_matches = handle_application_input();
    let matches = root_matches.subcommand_matches("createprotocolplugin").ok_or(format_err!("No subcommand found!"))?;
    let crate_name = matches.value_of("cratename").ok_or(format_err!("No crate name found!"))?;
    let protocol_filename = matches.value_of("protocol").ok_or(format_err!("No protocol file found!"))?;

    let mut crate_builder = ProtocolBufferCrate::new(crate_name, protocol_filename);
    crate_builder.create()?;

    Ok(())
}
