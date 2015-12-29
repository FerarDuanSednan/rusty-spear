extern crate clap;
use clap::{Arg, App, SubCommand};

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn create_structure(project : &str) -> Result<(), io::Error> {
    let base_path = Path::new(project);
    try!(fs::create_dir( base_path ));
    
    let src_path = base_path.join("src");
    try!(fs::create_dir( src_path.as_path() ));

    // src/controllers/
    try!(fs::create_dir( src_path.as_path().join("controllers").as_path() ));

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
    let mut f = try!(File::create( base_path.join("Cargo.toml") ));
    try!(f.write_all(format!("

[package]
name = \"{}\"
version = \"0.1.0\"
authors = [\"\"]

[dependencies]
nickel = \"*\"
    ", project).as_bytes())); 
   
    // main.rs
    f = try!(File::create( src_path.as_path().join("main.rs") ));
    try!(f.write_all(r#"
#[macro_use] extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));

    server.listen("127.0.0.1:3000");
}
    "#.as_bytes()));

    Ok(())
}

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("new")
                .about("create a new project")
                .arg(Arg::with_name("NAME")
                    .required(true)
                    ))
        .get_matches();

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("new") {
        println!("Do some stuff here : ");
        if matches.is_present("NAME") {
            // create here the project directories
            create_structure(matches.value_of("NAME").unwrap());
        }
    }

    // more program logic goes here...
}
