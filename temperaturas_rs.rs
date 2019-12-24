use std::io;
fn main() {

    let mut escala_inicial = String::new();
    let temp_final :f32 ;
    let mut k = String::new();
    
    println!("Programa de convertidor de temperaturas.");
    println!("Celsius, Fahrenheit, Kelvin y Rankine.");
    println!("Ingresar la escala inicial de temperatura en letra");

    io::stdin().read_line(&mut escala_inicial)
        .expect("Lo siento, no entendí eso muy bien");
    escala_inicial = escala_inicial.to_lowercase().trim().to_string();
    
    if escala_inicial == "celsius" {
        println!("Escogiste la escala {}", escala_inicial);
        println!("Ingresar la temperatura inicial en número (dígito)");
        let inicial = parsear_temp_inicial();
        if inicial > -274.0{
            let escala_final = escoger_escala();
            if escala_inicial != escala_final {
                if escala_final == "fahrenheit"{
                    temp_final = (inicial * 1.8) + 32.0;
                    println!("La temperatura final es {} grados", temp_final);
                } else if escala_final == "kelvin"{
                    temp_final = inicial + 273.15;
                    println!("La temperatura final es {} grados", temp_final);
                } else if escala_final == "rankine"{
                    temp_final = (inicial + 273.15) * 1.8;
                    println!("La temperatura final es {} grados", temp_final);
                } else {
                    println!("Ingresar una escala válida");
                }
            } else {
                println!("Ingresar una escala distinta a la inicial");
            }
        } else {
            println!("Ingresar una temperatura físicamente posible");
        }

    } else if escala_inicial == "fahrenheit" {
        println!("Escogiste la escala {}", escala_inicial);
        println!("Ingresar la temperatura inicial en número (dígito)");
        let inicial = parsear_temp_inicial();
        if inicial > -459.67{
            let escala_final = escoger_escala();
            if escala_inicial != escala_final {
                if escala_final == "celsius"{
                    temp_final = (inicial - 32.0) * 0.555 ;
                    println!("La temperatura final es {} grados", temp_final);
                } else if escala_final == "kelvin"{
                    temp_final = (inicial + 459.67 ) * 0.555 ;
                    println!("La temperatura final es {} grados", temp_final);
                } else if escala_final == "rankine"{
                    temp_final = inicial + 459.67 ;
                    println!("La temperatura final es {} grados", temp_final);
                } else {
                    println!("Ingresar una escala válida");
                }
                } else {
                    println!("Ingresar una escala distinta a la inicial");
                }
        } else {
            println!("Ingresar una temperatura físicamente posible");
        }

    } else if escala_inicial == "kelvin" {
        println!("Escogiste la escala {}", escala_inicial);
        println!("Ingresar la temperatura inicial en número (dígito)");
        let inicial = parsear_temp_inicial();
        if inicial > -1.0 {
            let escala_final = escoger_escala();
            if escala_inicial != escala_final {
            if escala_final == "celsius"{
                temp_final = inicial - 273.15 ;
                println!("La temperatura final es {} grados", temp_final);                 
            } else if escala_final == "fahrenheit"{
                temp_final = (inicial * 1.8) - 459.667 ;               
                println!("La temperatura final es {} grados", temp_final);
            }  else if escala_final == "rankine"{
                temp_final = inicial * 1.8 ;
                println!("La temperatura final es {} grados", temp_final);
            } else {
                println!("Ingresar una escala válida");
            }
            } else {
                println!("Ingresar una escala distinta a la inicial");
            }
        } else {
            println!("Ingresar una temperatura inicial en número (dígito)");
        } 

    } else if escala_inicial == "rankine" {
        println!("Escogiste la escala {}", escala_inicial);
        println!("Ingresar la temperatura inicial en número (dígito)");
        let inicial = parsear_temp_inicial();
        if inicial > -1.0 {
            let escala_final = escoger_escala();
            if escala_inicial != escala_final {
                if escala_final == "celsius" {
                    temp_final = (inicial - 491.667) / 1.8;
                    println!("La temperatura final es {} grados", temp_final);                    
                } else if escala_final == "fahrenheit" {
                    temp_final = inicial - 459.667;
                    println!("La temperatura final es {} grados", temp_final);
                } else if escala_final == "kelvin" {
                    temp_final = inicial / 1.8;
                    println!("La temperatura final es {} grados", temp_final);
                } else {
                    println!("Ingresar una escala válida");
                }
            } else {
                println!("Ingresar una escala distinta a la inicial");
            }
        } else {
            println!("Ingresar una temperatura inicial en número (dígito)");
        }
    } else {
        println!("Por favor ingresa una escala válida")
    }
    println!("Ingresar enter para salir");
    io::stdin().read_line(&mut k)
        .expect("Lo siento, no entendí eso muy bien");

}
fn parsear_temp_inicial() -> f32 {
    let mut temp_inicial = String::new();
    io::stdin().read_line(&mut temp_inicial)
        .expect("Lo siento, no entendí eso muy bien");
    let inicial: f32 = temp_inicial.trim().to_string().parse::<f32>().unwrap();
    println!("La temperatura inicial es de {} grados", inicial);
    return inicial;
  }
fn escoger_escala()-> String {
    let mut escala_final = String::new();
    println!("Ahora ingresar la escala a que se desea convertir");
    io::stdin().read_line(&mut escala_final)
        .expect("Lo siento, no entendí eso muy bien");
    escala_final = escala_final.to_lowercase().trim().to_string();
    println!("Escogiste la escala final {}", escala_final);
        return escala_final;
  }
