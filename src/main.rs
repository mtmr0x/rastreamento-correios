#[macro_use] extern crate nickel;

use nickel::{Nickkel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.get('*', middleware!("Hello Fucking World"));
    server.listen("127.0.0.1:6767");
}

