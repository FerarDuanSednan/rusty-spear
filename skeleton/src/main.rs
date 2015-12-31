#[macro_use] extern crate nickel;
extern crate latr;

use nickel::{Nickel, StaticFilesHandler};
use latr::Latr;

mod config;
mod controllers;

fn main() {

    Latr::set_host("127.0.0.1:3001");

    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("assets/"));
    server.utilize(config::routes::build());

    server.listen(Latr::get_host());
}
 
