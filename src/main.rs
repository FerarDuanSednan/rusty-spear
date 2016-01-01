extern crate clap;
extern crate walkdir;
extern crate git2;
extern crate tempdir;

use std::{io, fs, env};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Command;


use clap::{Arg, App, SubCommand};

use walkdir::WalkDir;

use git2::Repository;
use tempdir::TempDir;

const DEFAULT_SKELETON_PATH : &'static str = "https://github.com/FerarDuanSednan/rusp-skeleton.git";

struct Params<'a> {
    project_name : &'a str,
    project_path: &'a Path,
    skeleton_path: &'a str 
}

fn create_structure(params : &Params) -> Result<(), io::Error> {
    let base_path = params.project_path;

    let tmp_skeleton = clone_to_tmp(params.skeleton_path);

    let skeleton_path = tmp_skeleton.path();

    // Call to cargo to create base project
    let _ = Command::new("cargo").args(&["new", params.project_name]).output()
        .unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

    copy_skeleton( &skeleton_path, &base_path );

    // Add nickel dependency to Cargo.toml
    let mut f = try!(OpenOptions::new().write(true).append(true)
                     .open( base_path.join("Cargo.toml").as_path() ));
    try!(f.write_all(
br#"
nickel = "*"

[dependencies.rusp]
git = "https://github.com/FerarDuanSednan/rusty-spear.git"
"#));

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

fn clone_to_tmp(url : &str) -> TempDir {

    let tempdir = match TempDir::new("rusp") {
        Ok(t) => t,
        Err(e) => panic!("Failure creating directory : {}", e)
    };

    match Repository::clone(url, tempdir.path()) {
        Err(e) => {
            let _ = tempdir.close(); // don't really care about this
            panic!("Fail to clone the repository : {}", e);
        },
        _ => tempdir
    }
}

/**
 * TODO Check if the plugin exists
 * TODO Before copying, check if we don't override something
 */
fn install_plugin(plugins_path : &Path, plugin_name : &str, project_path : &Path) {
    let plugin_path = plugins_path.join(plugin_name);
    copy_skeleton(plugin_path.as_path(), project_path);
}

fn main() {
    let matches = App::new("rusp")
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
                         .help("Set the path to the skeleton (url to git repository).")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("install")
                    .about("Install a plugin")
                    .arg(Arg::with_name("PLUGINS_NAMES")
                         .help("Name of the plugin to install")
                         .multiple(true)
                         .required(true))
                    .arg(Arg::with_name("PLUGINS_PATH")
                         .long("plugins-path")
                         .help("Set the path to the plugins folder.")
                         .takes_value(true)))
        .get_matches();

    // ----- new -----
    if let Some(matches) = matches.subcommand_matches("new") {
        let _ = Command::new("cargo").arg("--version").output().unwrap_or_else(|e| { panic!("Failed to execute cargo: {}", e) });

        // NAME is present
        if matches.is_present("NAME") {
            let name = matches.value_of("NAME").unwrap();
            let project_path = env::current_dir().unwrap().join(name);
            let mut skeleton_path = DEFAULT_SKELETON_PATH;

            if matches.is_present("SKEL_PATH") {
                skeleton_path = matches.value_of("SKEL_PATH").unwrap();
            }

            let params = Params {project_name : name, skeleton_path : skeleton_path, project_path: project_path.as_path() };
            // TODO: check if path are real directories

            create_structure(&params).expect("Failure creating structure.");
        }
    }

    // ----- install -----
    if let Some(matches) = matches.subcommand_matches("install") {

        if matches.is_present("PLUGINS_NAMES") {
            let plugins = matches.values_of("PLUGINS_NAMES").unwrap();

            let mut plugins_path = Path::new("plugins");
            let project_path = env::current_dir().unwrap();
            if matches.is_present("PLUGINS_PATH") {
                plugins_path = Path::new(matches.value_of("PLUGINS_PATH").unwrap());
            }

            for plugin in plugins {
                install_plugin(plugins_path, plugin, project_path.as_path());
            }
        }
    }

}

