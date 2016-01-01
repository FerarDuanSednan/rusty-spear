extern crate rusp;

use rusp::RuspApp;

#[test]
fn rust_app_persistance() {
    assert_eq!(RuspApp::get_host(), "127.0.0.1:3000");

    RuspApp::set_host("127.0.0.1:3001");
    assert_eq!(RuspApp::get_host(), "127.0.0.1:3001");
}
