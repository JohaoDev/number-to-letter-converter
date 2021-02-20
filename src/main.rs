#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

extern crate serde;
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Data {
    number: usize,
}

fn main() {
    // let test = String::from("100000000000000000000000000000000000000"); //39
    // let test: i128 = 1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000; //121
    // println!("{}", test.len());

    let mut server = Nickel::new();

    server.post( "/number", middleware!( |request| {
        let data = request.json_as::<Data>().unwrap();
        let number: usize = data.number;

        // let response = f_decenas(number).to_string();
        // let response = f_seccion(number, 1000, String::from("UN MIL"), String::from("MIL"));
        let response = f_millones(number);

        format!("{}", response)
    }));

    server.listen("127.0.0.1:8080").unwrap();
}

fn f_unidades(num: usize) -> String {
    let mut result = String::from("");

    match num {
        1 => result = "UNO".to_string(),
        2 => result = "DOS".to_string(),
        3 => result = "TRES".to_string(),
        4 => result = "CUATRO".to_string(),
        5 => result = "CINCO".to_string(),
        6 => result = "SEIS".to_string(),
        7 => result = "SIETE".to_string(),
        8 => result = "OCHO".to_string(),
        9 => result = "NUEVE".to_string(),
        _ => result = "".to_string(),
    }
    result
}

fn f_decenas(num: usize) -> String {
    let mut result = String::from("");
    let decena = num / 10;
    let unidad: usize = num - (decena * 10);

    match decena {
        1 => match unidad {
            0 => result = "DIEZ".to_string(),
            1 => result = "ONCE".to_string(),
            2 => result = "DOCE".to_string(),
            3 => result = "TRECE".to_string(),
            4 => result = "CATORCE".to_string(),
            5 => result = "QUINCE".to_string(),
            _ => {
                let mut temp = String::from("DIECI");
                temp.push_str(&f_unidades(unidad));
                result = temp
            },
        }
        2 => match unidad {
            0 => result = "VEINTE".to_string(),
            _ => {
                let mut temp = String::from("VEINTI");
                temp.push_str(&f_unidades(unidad));
                result = temp
            },
        }
        3 => result = f_decenas_y("TREINTA".to_string(), unidad),
        4 => result = f_decenas_y("CUARENTA".to_string(), unidad),
        5 => result = f_decenas_y("CINCUENTA".to_string(), unidad),
        6 => result = f_decenas_y("SESENTA".to_string(), unidad),
        7 => result = f_decenas_y("SETENTA".to_string(), unidad),
        8 => result = f_decenas_y("OCHENTA".to_string(), unidad),
        9 => result = f_decenas_y("NOVENTA".to_string(), unidad),
        _ => result = f_unidades(unidad),
    }
    result
}

fn f_decenas_y(string_singular: String, num_unidades: usize) -> String {
    let mut result = String::from("");
    
    if num_unidades > 0 {
        result.push_str(&string_singular);
        result.push_str(" Y ");
        result.push_str(&f_unidades(num_unidades));
    } else {
        result.push_str(&string_singular);
    }
    result
}

fn f_centenas(num: usize) -> String {
    let centenas = num / 100;
    let decena = num - (centenas * 100);
    let mut result = String::from("");

    match centenas {
        1 => {
            if decena > 0 {
                result.push_str("CIENTO ");
                result.push_str(&f_decenas(decena));
            } else {
                result.push_str("CIEN");
            }
        }
        2 => {
            result.push_str("DOSCIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        3 => {
            result.push_str("TRESCIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        4 => {
            result.push_str("CUATROCIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        5 => {
            result.push_str("QUINIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        6 => {
            result.push_str("SEISCIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        7 => {
            result.push_str("SETECIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        8 => {
            result.push_str("OCHOCIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        9 => {
            result.push_str("NOVECIENTOS ");
            result.push_str(&f_decenas(decena));
        }
        _ => result.push_str(&f_decenas(decena)),
    }
    result
}

fn f_seccion(num: usize, divisor: usize, string_singular: String, string_plural: String) -> String {
    let mut result = String::from("");
    let cientos = num / divisor;
    let resto = num - (cientos * divisor);

    if cientos > 0 {
        if cientos > 1 {
            result.push_str(&f_centenas(cientos));
            result.push_str(" ");
            result.push_str(&string_plural);
        } else {
            result.push_str(&string_singular);
        }
    }

    if resto > 0 {
        result += "";
    }
    result
}

fn f_miles(num: usize) -> String {
    let mut result = String::from("");
    let divisor = 1000;
    let cientos = num / divisor;
    let resto = num - (cientos * divisor);

    let string_miles = f_seccion(num, divisor, String::from("MIL"), String::from("MIL"));
    let string_centenas = f_centenas(resto);
    
    if string_miles == "" {
        result.push_str(&string_centenas);
    } else {
        result.push_str(&string_miles);
        result.push_str(" ");
        result.push_str(&string_centenas);
    }
    result
}

fn f_millones(num: usize) -> String {
    let mut result = String::from("");
    let divisor = 1000000;
    let cientos = num / divisor;
    let resto = num - (cientos * divisor);

    let string_millones = f_seccion(num, divisor, String::from("UN MILLON"), String::from("MILLONES"));
    let string_miles = f_miles(resto);

    if string_millones == "" {
        result.push_str(&string_miles);
    } else {
        result.push_str(&string_millones);
        result.push_str(" ");
        result.push_str(&string_miles);
    }
    result
}
