extern crate latr;

use latr::Latr;

#[test]
fn access_host_setting() {
    assert_eq!(Latr::get_host(), "127.0.0.1:3000" );

    Latr::set_host("127.0.0.1:3001");
    assert_eq!(Latr::get_host(), "127.0.0.1:3001" );
}


