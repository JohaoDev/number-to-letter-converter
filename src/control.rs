mod cardinals;
mod logic;

pub fn control(number_string: String) -> String {
    let mut result = String::from("");
    let mut result_vec_temp: Vec<String> = Vec::new();
    let number_vector_bytes = number_string.as_bytes();
    
    let mut number_len = number_string.len();
    let mut counter_while = 0;
    let mut counter_if = 0;
    let mut counter_len = 1;
    let mut vector_numeros: Vec<char> = Vec::new();
    let mut vector_temp: Vec<char> = Vec::new();

    for elem in number_vector_bytes {
        vector_numeros.push(*elem as char);
    }

    while counter_while < number_len {
        if counter_if < 3 {
            let char_temp = vector_numeros[number_len-counter_len];
            vector_temp.push(char_temp);
            vector_numeros.pop();
            
            counter_if += 1;
            counter_len += 1;
        }
        
        let vector_temp_len = vector_temp.len();

        if vector_temp_len == 3 {
            let mut result: String = String::from("");

            let temp1 = vector_temp[0].to_string();
            let temp2 = vector_temp[1].to_string();
            let temp3 = vector_temp[2].to_string();

            result.push_str(&temp3);
            result.push_str(&temp2);
            result.push_str(&temp1);

            let number: i128 = result.parse().unwrap();

            let response = cardinals::f_centenas(number);

            // println!("{:?}", response);
            result_vec_temp.push(response);
            vector_temp.pop();
            vector_temp.pop();
            vector_temp.pop();

            counter_if = 0;
        } 

        counter_while += 1;
    }
    // println!("{:?}", result_vec_temp);
    result = logic::sort_vector(result_vec_temp);
    result
}
