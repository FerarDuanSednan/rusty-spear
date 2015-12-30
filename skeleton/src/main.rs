#[macro_use] extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};

mod config;
mod controllers;

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));
    server.utilize(config::routes::build());

    server.listen("127.0.0.1:3000");
}
 
