mod cardinals;

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Data {
    number: i128,
}

fn main() {
    let mut server = Nickel::new();

    server.post( "/number", middleware!( |request| {
        let data = request.json_as::<Data>().unwrap();
        let number = data.number;
        
        let response = cardinals::f_sextillones(number);
        
        format!("{}", response)
    }));

    server.listen("127.0.0.1:8000").unwrap();
}
