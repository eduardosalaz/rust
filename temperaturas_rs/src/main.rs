use std::io;
fn main() {

    let mut initial_scale = String::new();
    let final_temp :f32 ;
    let mut k = String::new();
    
    println!("Temperature converter program.");
    println!("Celsius, Fahrenheit, Kelvin and Rankine.");
    println!("Enter the name of the initial scale.");

    io::stdin().read_line(&mut initial_scale)
        .expect("Sorry, didn't quite catch that.");
    initial_scale = initial_scale.to_lowercase().trim().to_string();
    
    if initial_scale == "celsius" {
        println!("You chose the {} scale", initial_scale);
        println!("Now enter the initial temperature in numbers.");
        let inicial = parse_initial_temp();
        if inicial > -274.0{
            let final_scale = choose_scale();
            if initial_scale != final_scale {
                if final_scale == "fahrenheit"{
                    final_temp = (inicial * 1.8) + 32.0;
                    println!("The final temperature in degrees is {}", final_temp);
                } else if final_scale == "kelvin"{
                    final_temp = inicial + 273.15;
                    println!("The final temperature in degrees is {}", final_temp);
                } else if final_scale == "rankine"{
                    final_temp = (inicial + 273.15) * 1.8;
                    println!("The final temperature in degrees is {}", final_temp);
                } else {
                    println!("Please enter a valid scale");
                }
            } else {
                println!("Please enter a scale different to the original");
            }
        } else {
            println!("Please enter a physicically valid temperature");
        }

    } else if initial_scale == "fahrenheit" {
        println!("You chose the {} scale", initial_scale);
        println!("Now enter the initial temperature in numbers.");
        let inicial = parse_initial_temp();
        if inicial > -459.67{
            let final_scale = choose_scale();
            if initial_scale != final_scale {
                if final_scale == "celsius"{
                    final_temp = (inicial - 32.0) * 0.555 ;
                    println!("The final temperature in degrees is {}", final_temp);
                } else if final_scale == "kelvin"{
                    final_temp = (inicial + 459.67 ) * 0.555 ;
                    println!("The final temperature in degrees is {}", final_temp);
                } else if final_scale == "rankine"{
                    final_temp = inicial + 459.67 ;
                    println!("The final temperature in degrees is {}", final_temp);
                } else {
                    println!("Please enter a valid scale");
                }
                } else {
                    println!("Please enter a scale different to the original");
                }
        } else {
            println!("Please enter a physicically valid temperature");
        }

    } else if initial_scale == "kelvin" {
        println!("You chose the {} scale", initial_scale);
        println!("Now enter the initial temperature in numbers.");
        let inicial = parse_initial_temp();
        if inicial > -1.0 {
            let final_scale = choose_scale();
            if initial_scale != final_scale {
            if final_scale == "celsius"{
                final_temp = inicial - 273.15 ;
                println!("The final temperature in degrees is {}", final_temp);                 
            } else if final_scale == "fahrenheit"{
                final_temp = (inicial * 1.8) - 459.667 ;               
                println!("The final temperature in degrees is {}", final_temp);
            }  else if final_scale == "rankine"{
                final_temp = inicial * 1.8 ;
                println!("The final temperature in degrees is {}", final_temp);
            } else {
                println!("Please enter a valid scale");
            }
            } else {
                println!("Please enter a scale different to the original");
            }
        } else {
            println!("Ingresar una temperatura inicial en número (dígito)");
        } 

    } else if initial_scale == "rankine" {
        println!("You chose the {} scale", initial_scale);
        println!("Now enter the initial temperature in numbers.");
        let inicial = parse_initial_temp();
        if inicial > -1.0 {
            let final_scale = choose_scale();
            if initial_scale != final_scale {
                if final_scale == "celsius" {
                    final_temp = (inicial - 491.667) / 1.8;
                    println!("The final temperature in degrees is {}", final_temp);                    
                } else if final_scale == "fahrenheit" {
                    final_temp = inicial - 459.667;
                    println!("The final temperature in degrees is {}", final_temp);
                } else if final_scale == "kelvin" {
                    final_temp = inicial / 1.8;
                    println!("The final temperature in degrees is {}", final_temp);
                } else {
                    println!("Please enter a valid scale");
                }
            } else {
                println!("Please enter a scale different to the original");
            }
        } else {
            println!("Ingresar una temperatura inicial en número (dígito)");
        }
    } else {
        println!("Por favor ingresa una escala válida")
    }
    println!("Press a key to exit");
    io::stdin().read_line(&mut k)
        .expect("Sorry, didn't quite catch that.");

}
fn parse_initial_temp() -> f32 {
    let mut temp_inicial = String::new();
    io::stdin().read_line(&mut temp_inicial)
        .expect("Sorry, didn't quite catch that.");
    let inicial: f32 = temp_inicial.trim().to_string().parse::<f32>().unwrap();
    println!("Initial temperature is {} degrees", inicial);
    return inicial;
  }
fn choose_scale()-> String {
    let mut final_scale = String::new();
    println!("Now enter the desired scale to convert");
    io::stdin().read_line(&mut final_scale)
        .expect("Sorry, didn't quite catch that.");
    final_scale = final_scale.to_lowercase().trim().to_string();
    println!("You chose scale{}", final_scale);
        return final_scale;
  }
