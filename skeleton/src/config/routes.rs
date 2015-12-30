
use controllers::web_controller;

use nickel::{Nickel, Router, HttpRouter};

pub fn build() -> Router {
    let mut router = Nickel::router();
    router.get("/", web_controller::index);
    router
}

