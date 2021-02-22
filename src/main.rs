mod control;
mod cardinals;

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Data {
    number: String,
}

fn main() {
    let mut server = Nickel::new();

    server.post( "/number", middleware!( |request| {
        let response;
        let data = request.json_as::<Data>().unwrap();
        let number = data.number;

        if number.len() < 3 {
            response = cardinals::f_centenas(number.parse().unwrap());
        } else {
            response = control::control(number);
        }


        format!("{}", response)
    }));

    server.listen("127.0.0.1:8000").unwrap();
}
