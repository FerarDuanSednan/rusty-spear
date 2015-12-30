extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use std::process::Command;

const TEMPLATE_PATH : &'static str = "./templates";

fn create_structure(project : &str) -> Result<(), io::Error> {
    let base_path = Path::new(project);

    // Call to cargo to create base project
    let _ = Command::new("cargo").args(&["new", project]).output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

    let src_path = base_path.join("src");

    // src/controllers/
    try!(fs::create_dir( src_path.as_path().join("controllers").as_path() ));

    // src/config/
    try!(fs::create_dir( src_path.as_path().join("config").as_path() ));

    // src/models/
    try!(fs::create_dir( src_path.as_path().join("models").as_path() ));

    // src/helpers/
    try!(fs::create_dir( src_path.as_path().join("helpers").as_path() ));

    // views/
    try!(fs::create_dir( base_path.join("views").as_path() ));

    // assets/{javascripts,stylesheets,images}
    let assets_path = base_path.join("assets");
    try!(fs::create_dir( assets_path.as_path() ));
    try!(fs::create_dir( assets_path.as_path().join("javascripts").as_path() ));
    try!(fs::create_dir( assets_path.as_path().join("stylesheets").as_path() ));
    try!(fs::create_dir( assets_path.as_path().join("images").as_path() ));

    // populate with files
    // Cargo.toml
    let mut f = try!(OpenOptions::new().write(true).append(true).open( base_path.join("Cargo.toml").as_path() ));
    try!(f.write_all(b"nickel = \"*\"\n\n"));

    // main.rs
    try!(fs::copy( Path::new(TEMPLATE_PATH).join("main.rs").as_path(), src_path.join("main.rs").as_path() ));

    Ok(())
}

fn main() {
    let matches = App::new("lart")
        .version("1.0")
        .author("Arnaud Fernandés <arnaud.fernandes@laposte.net>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("new")
                    .about("Create a new project")
                    .arg(Arg::with_name("NAME")
                         .required(true)
                        ))
        .get_matches();

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("new") {
        let _ = Command::new("cargo").arg("--version").output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

        // Encode the resulting data.
        if matches.is_present("NAME") {
            // create here the project directories
            create_structure(matches.value_of("NAME").unwrap());
        }
    }

    // more program logic goes here...
}

