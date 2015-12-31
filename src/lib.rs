#[macro_use] extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref L : Mutex<Latr> = Mutex::new(Latr::new());
}

pub struct Latr {
    host: &'static str,
}

impl Latr {

    pub fn new() -> Latr {
        Latr { host: "127.0.0.1:3000" }
    }

    pub fn get_host() -> &'static str {
        let single = self::L.lock().unwrap();
        single.host
    }

    pub fn set_host( host: &'static str) {
        let mut single = self::L.lock().unwrap();
        single.host = host;
    }

}

