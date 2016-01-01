#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref APP : Mutex<RuspApp<'static>> = Mutex::new(RuspApp::new());
}

pub struct RuspApp<'r> {
    host: &'r str
}

impl<'r> RuspApp<'r> {

    fn new() -> RuspApp<'r> {
        RuspApp {
            host: "127.0.0.1:3000",
        }
    }

    pub fn get_host() -> &'r str {
        APP.lock().unwrap().host.clone()
    }

    pub fn set_host(host : &'static str) {
        APP.lock().unwrap().host = host;
    }

    //----- Accessors -----
/*
    pub fn get_host(&self) -> &str {
        self.host.clone()
    }

    pub fn set_host(&mut self, host: &'r str) {
        self.host = host.clone();
    }
*/


}
