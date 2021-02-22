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

    server.get("/", middleware!("Welcome"));
    server.post( "/number", middleware!( |request| {
        let response;
        let data = request.json_as::<Data>().unwrap();
        let number = data.number;

        if number.len() < 20 {
            response = cardinals::f_logic(number.parse().unwrap());
        } 
        else {
            response = control::control(number);
        }

        format!("{}", response)
    }));

    server.listen("127.0.0.1:8000").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(cardinals::f_logic(1), "UNO");
        assert_eq!(cardinals::f_logic(100), "CIEN");
        assert_eq!(cardinals::f_logic(1000000), "UN MILLON ");
        assert_eq!(cardinals::f_logic(1000000000000000000), "UN TRILLON ");
    }
}