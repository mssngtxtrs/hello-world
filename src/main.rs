//#############
//#   CONST   #
//#############



const CELSIUS_TO_FAHRENHEIT: f64 = 9.0 / 5.0;
const FAHRENHEIT_OFFSET: f64 = 32.0;
const KELVIN_OFFSET: f64 = 273.15;



//###############
//#   IMPORTS   #
//###############



use std::io;
use std::io::Write;
use std::str::FromStr;
use std::fmt::Display;



//############
//#   MAIN   #
//############



fn main() {
    //--Info about operations--
    print("Temperature converter
    1. Celsius to Fahrenheit
    2. Celsius to Kelvin
    3. Fahrenheit to Celsius
    4. Kelvin to Celsius
    5. Exit", true, false);

    //--Getting value loop--
    loop {
        //--Variable for storing temperature--
        let temperature: f64;

        print("\nChoose operation (1-5): ", false, false);

        //--Getting operation choice--
        match get_value() {
            //--Celsius to Fahrenheit--
            1 => {
                print("Enter temperature in °C: ", false, false);
                temperature = get_value();
                print(format!("{}°C is {}°F", temperature, temperature * CELSIUS_TO_FAHRENHEIT + FAHRENHEIT_OFFSET), true, false);
            },

            //--Celsius to Kelvin--
            2 => {
                print("Enter temperature in °C: ", false, false);
                temperature = get_value();
                print(format!("{}°C is {}K", temperature, temperature + KELVIN_OFFSET), true, false);
            },

            //--Fahrenheit to Celsius--
            3 => {
                print("Enter temperature in °F: ", false, false);
                temperature = get_value();
                print(format!("{}°F is {}°C", temperature, (temperature - FAHRENHEIT_OFFSET) / CELSIUS_TO_FAHRENHEIT), true, false);
            },

            //--Kelvin to Celsius--
            4 => {
                print("Enter temperature in K: ", false, false);
                temperature = get_value();
                print(format!("{}K is {}°C", temperature, temperature - KELVIN_OFFSET), true, false);
            },

            //--Exit--
            5 => {
                print("Exiting program.", true, false);
                break;
            },

            //--Invalid choice--
            _ => println!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }
}



//#################
//#   FUNCTIONS   #
//#################



//----Get needed value from string----
fn get_value<T: FromStr>() -> T {
    //--Buffer for storing input--
    let mut buffer = String::new();

    loop {
        //--Read line from stdin--
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                //--Trimming and parsing input--
                match buffer.trim().parse::<T>() {
                    Ok(num) => return num,
                    _ => {
                        print("Invalid data type. Please try again.", true, true);
                        buffer.clear();
                    }
                }
            },
            Err(e) => {
                print(format!("Error reading input: {}. Please try again.", e), true, true);
            }
        }
    };
}

//----Print function that i find more convenient----
fn print<T: Display>(display: T, newline: bool, error: bool) {
    //--Printing to stdout or stderr--
    if error {
        eprint!("{}", display);
    } else {
        print!("{}", display);
    }

    //--Newline--
    if newline {
        print!("\n");
    }

    //--Flushing output (to ensure prompt appears before input)--
    io::stdout().flush().unwrap();
}