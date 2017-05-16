#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter, MediaType};
use rustc_serialize::json::{Json, ToJson};

mod controllers;

use controllers::cep_consult::cep_consult_controller::receive_cep;

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!("Hello Fucking World"));

    server.get("/cep/:cepnumber", middleware! { |req, mut res|
        let cep_number_param = req.param("cepnumber").unwrap();
        res.set(MediaType::Json);

        println!("cep is {:?}", receive_cep(cep_number_param.to_string()));
        receive_cep(cep_number_param.to_string());
    });

    server.listen("127.0.0.1:6767");
}

