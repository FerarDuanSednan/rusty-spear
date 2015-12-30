extern crate clap;
extern crate walkdir;

use clap::{Arg, App, SubCommand};

use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use std::env;

use std::process::Command;

use walkdir::WalkDir;

struct Params<'a> {
    project_name : &'a str,
    project_path: &'a Path,
    skeleton_path: &'a Path
}

fn create_structure(params : &Params) -> Result<(), io::Error> {
    let base_path = params.project_path;
    let skeleton_path = params.skeleton_path;


    // Call to cargo to create base project
    let _ = Command::new("cargo").args(&["new", params.project_name]).output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

    copy_skeleton( &skeleton_path, &base_path );
 
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

fn copy_skeleton(skeleton_path : &Path, project_path : &Path) {

    let mut relative_path = project_path.to_path_buf();
    let mut prev_depth = 0;
    for entry in WalkDir::new(skeleton_path).follow_links(false).min_depth(1) {
        let entry = entry.unwrap();

        if entry.depth() <= prev_depth {
            for _ in 0..(prev_depth - entry.depth() +1) {
                relative_path.pop();
            }
        }

        relative_path.push(entry.file_name());

        prev_depth = entry.depth();

        if entry.file_type().is_dir() {
            mkdir( relative_path.as_path() ).expect("Fail to write directory");
        } else if entry.file_type().is_file() {
            cp(entry.path(), relative_path.as_path() ).expect("Fail to write file");
        }
    }
}

fn main() {
    let matches = App::new("lart")
        .version("0.1")
        .author("Arnaud Fernandés <arnaud.fernandes@laposte.net>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("new")
                    .about("Create a new project")
                    .arg(Arg::with_name("NAME")
                         .help("Name of the project")
                         .required(true))
                    .arg(Arg::with_name("SKEL_PATH")
                         .long("skeleton")
                         .help("Set the path to the skeleton folder.")
                         .takes_value(true)))
        .get_matches();

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("new") {
        let _ = Command::new("cargo").arg("--version").output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

        // Encode the resulting data.
        if matches.is_present("NAME") {
            let name = matches.value_of("NAME").unwrap();
            let project_path = env::current_dir().unwrap().join(name);
            let mut skeleton_path = env::current_dir().unwrap().join("skeleton"); 

            if matches.is_present("SKEL_PATH") {
                let skel_path = matches.value_of("SKEL_PATH").unwrap();
                skeleton_path = env::current_dir().unwrap().join(skel_path);
            }

            let params = Params {project_name : name, skeleton_path : skeleton_path.as_path(), project_path: project_path.as_path() };
            // TODO: check if path are real directories

            create_structure(&params).expect("Failure creating structure.");
        }
    }
}

