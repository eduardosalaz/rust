use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main (){

    let numero_secreto = rand::thread_rng().gen_range(1, 10);
    
    println!("Adivina el número en el que estoy pensando");
    println!("Estoy pensando en un número entre el 1 y el 10");
    println!("¿Puedes adivinarlo?");
    
    loop {
        let mut adivinanza = String::new();
        
        io::stdin().read_line(&mut adivinanza)
            .expect("Lo siento, no entendí eso muy bien");
            
        let adivinanza: u32 = match adivinanza.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
        
        println!("Adivinaste: {}", adivinanza);
        
        match adivinanza.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado pequeño"),
            Ordering::Greater => println!("Demasiado grande"),
            Ordering::Equal => {
                println!("Ganaste");
                println!("El número en el que estaba pensando es {}", numero_secreto);
                break;
            }
        }
}
}
