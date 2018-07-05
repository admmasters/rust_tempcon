use std::io;

fn to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn select_input_format() -> char {
    println!("Are you converting from C or F?");

    let mut format = String::new();

    io::stdin()
        .read_line(&mut format)
        .expect("Was not a string");

    match format.trim().as_ref() {
        "c" => 'c',
        "f" => 'f',
        &_ => {
            println!("We only accept 'c' or 'f'");
            select_input_format()
        }
    }
}

fn select_temp_value() -> Option<f64> {
    println!("What is the input temperature?");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Was not a string");
    match temp.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn temp_suffix(input_format: char) -> String {
    match input_format {
        'c' => String::from("°F"),
        'f' => String::from("°C"),
        _ => panic!("Impossible"),
    }
}

fn convert_temperature(format: char) -> bool {
    let temp = select_temp_value();
    match temp {
        Some(temp) => {
            let converted_temp = if format == 'c' {
                to_fahrenheit(temp)
            } else {
                to_celsius(temp)
            };
            println!("Converted temp: {}{}", converted_temp, temp_suffix(format));
            true
        }
        None => false,
    }
}

fn ask_questions(format: char) {
    if !convert_temperature(format) {
        ask_questions(format)
    }
}

fn main() {
    println!("Welcome to the temperature converter");
    let format = select_input_format();
    ask_questions(format);
}
