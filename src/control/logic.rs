pub fn sort_vector(vector: Vec<String>) -> String {
    let mut result: String = String::from("");
    let mut counter = 0;
    let mut vector_counter_len = 1;
    let vector_len = vector.len();
    
    let mut vector_temp: Vec<String> = Vec::new();

    match vector_len {
        1 => result.push_str(&vector[0].to_string()),
        2 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
        },
        3 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
        },
        4 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
        },
        5 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
        },
        6 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
        },
        7 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
        },
        8 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
        },
        9 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
        },
        10 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
        },
        11 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
        },
        12 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
        },
        13 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
        },
        14 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
        },
        15 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
        },
        16 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
        },
        17 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
        },
        18 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
        },
        19 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
        },
        20 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
        },
        // START

        21 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
        },
        22 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
        },
        23 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[21].to_string());
        },
        24 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[22].to_string());
        },
        25 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[23].to_string());
        },
        26 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[24].to_string());
        },
        27 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[25].to_string());
        },
        28 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[25].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[26].to_string());
        },
        29 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" CUATORDECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[25].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[26].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[27].to_string());
        },
        30 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" CUATORDECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[25].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[26].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[27].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[28].to_string());
        },
        31 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" QUINDECILLON ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" CUATORDECILLON ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[25].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[26].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[27].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[28].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[29].to_string());
        },
        32 => {
            while counter < vector_len {
                let temp = &vector[vector_len-vector_counter_len];

                vector_temp.push(temp.to_string());
        
                counter += 1;
                vector_counter_len += 1;
            }
            result.push_str(&vector_temp[0].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[1].to_string());
            result.push_str(" QUINDECILLON ");
            result.push_str(&vector_temp[2].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[3].to_string());
            result.push_str(" CUATORDECILLON ");
            result.push_str(&vector_temp[4].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[5].to_string());
            result.push_str(" TREDECILLON ");
            result.push_str(&vector_temp[6].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[7].to_string());
            result.push_str(" DUODECILLON ");
            result.push_str(&vector_temp[8].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[9].to_string());
            result.push_str(" UNDECILLON ");
            result.push_str(&vector_temp[10].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[11].to_string());
            result.push_str(" DECILLON ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[12].to_string());
            result.push_str(" NONILLONES ");
            result.push_str(&vector_temp[13].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[14].to_string());
            result.push_str(" OCTILLONES ");
            result.push_str(&vector_temp[15].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[16].to_string());
            result.push_str(" SEPTILLONES ");
            result.push_str(&vector_temp[17].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[18].to_string());
            result.push_str(" SEXTILLONES ");
            result.push_str(&vector_temp[19].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[20].to_string());
            result.push_str(" QUINTILLONES ");
            result.push_str(&vector_temp[21].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[22].to_string());
            result.push_str(" CUATRILLONES ");
            result.push_str(&vector_temp[23].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[24].to_string());
            result.push_str(" TRILLONES ");
            result.push_str(&vector_temp[25].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[26].to_string());
            result.push_str(" BILLONES ");
            result.push_str(&vector_temp[27].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[28].to_string());
            result.push_str(" MILLONES ");
            result.push_str(&vector_temp[29].to_string());
            result.push_str(" MIL ");
            result.push_str(&vector_temp[30].to_string());
        },
        // 33 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        // },
        // 34 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        // },
        // 35 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" SEPTILLONES ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[13].to_string());
        // },
        // 36 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" SEPTILLONES ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[13].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[14].to_string());
        // },
        // 37 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" OCTILLONES ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" SEPTILLONES ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[13].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[14].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[15].to_string());
        // },
        // 38 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" OCTILLONES ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" SEPTILLONES ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[13].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[14].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[15].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[16].to_string());
        // },
        // 39 => {
        //     while counter < vector_len {
        //         let temp = &vector[vector_len-vector_counter_len];

        //         vector_temp.push(temp.to_string());
        
        //         counter += 1;
        //         vector_counter_len += 1;
        //     }
        //     result.push_str(&vector_temp[0].to_string());
        //     result.push_str(" NONILLONES ");
        //     result.push_str(&vector_temp[1].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[2].to_string());
        //     result.push_str(" OCTILLONES ");
        //     result.push_str(&vector_temp[3].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[4].to_string());
        //     result.push_str(" SEPTILLONES ");
        //     result.push_str(&vector_temp[5].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[6].to_string());
        //     result.push_str(" SEXTILLONES ");
        //     result.push_str(&vector_temp[7].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[8].to_string());
        //     result.push_str(" QUINTILLONES ");
        //     result.push_str(&vector_temp[9].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[10].to_string());
        //     result.push_str(" CUATRILLONES ");
        //     result.push_str(&vector_temp[11].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" TRILLONES ");
        //     result.push_str(&vector_temp[12].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[13].to_string());
        //     result.push_str(" BILLONES ");
        //     result.push_str(&vector_temp[14].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[15].to_string());
        //     result.push_str(" MILLONES ");
        //     result.push_str(&vector_temp[16].to_string());
        //     result.push_str(" MIL ");
        //     result.push_str(&vector_temp[17].to_string());
        // },


        // END

        _ => println!("Ingrese un número"),
    }



    result
}