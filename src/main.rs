extern crate clap;
extern crate walkdir;

use clap::{Arg, App, SubCommand};

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use std::process::Command;

use walkdir::WalkDir;

fn create_structure(project : &str) -> Result<(), io::Error> {
    let base_path = Path::new(project);

    // Call to cargo to create base project
    let _ = Command::new("cargo").args(&["new", project]).output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

    match base_path.to_str() {
        Some(path) => copy_skeleton("skeleton", path),
        None => panic!("Failed to copy skeleton.")
    }

    // Add nickel dependency to Cargo.toml
    let mut f = try!(OpenOptions::new().write(true).append(true).open( base_path.join("Cargo.toml").as_path() ));
    try!(f.write_all(b"nickel = \"*\"\n\n"));

    Ok(())
}

fn mkdir(dir: &Path) -> io::Result<()> {
    if fs::metadata(&dir).is_err() {
        try!(fs::create_dir(dir));
    }
    Ok(())
}

fn cp(from : &Path,  to: &Path) -> io::Result<()> {
    try!(fs::copy(from, to));
    Ok(())
}

fn copy_skeleton(skeleton_path : &str, project_path : &str) {
    let project = Path::new(project_path);

    for entry in WalkDir::new(skeleton_path).follow_links(false).min_depth(1) {
        let entry = entry.unwrap();

        let mut comps = entry.path().components();
        comps.next();
        let relative_path = comps.as_path();

        if entry.file_type().is_dir() {
            mkdir( project.join(relative_path).as_path()  );
        } else if entry.file_type().is_file() {
            cp(entry.path(), project.join(relative_path).as_path() );

        }
    }
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

