use std::io;
fn main() {

    let mut escala_inicial = String::new();
    let mut escala_final = String::new();
    let mut temp_inicial = String::new();
    let mut temp_final = String::new();

    println!("Programa de convertidor de temperaturas.");
    println!("Celsius, Fahrenheit, Kelvin y Rankine.");
    println!("Ingresar la escala inicial de temperatura en letra");

    io::stdin().read_line(&mut escala_inicial)
        .expect("Lo siento, no entendí eso muy bien");
    escala_inicial = escala_inicial.to_lowercase().trim().to_string();
    
    if escala_inicial == "celsius" {
        imprimir_inicial(escala_inicial);
        io::stdin().read_line(&mut temp_inicial);
        let inicial: f64 = temp_inicial.trim().parse();
    } else if escala_inicial == "fahrenheit" {
        imprimir_inicial(escala_inicial);
    } else if escala_inicial == "kelvin" {
        imprimir_inicial(escala_inicial);
    } else if escala_inicial == "rankine" {
        imprimir_inicial(escala_inicial);
    } else {
        println!("Por favor ingresa una escala válida")
    }
}

fn imprimir_inicial(escala_inicial: String) {
    println!("Escogiste la escala {}", escala_inicial);
    println!("Ingresar la temperatura inicial en número (dígito)")
}
