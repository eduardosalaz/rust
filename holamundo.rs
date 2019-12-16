fn main() {
  println!("Hola mundo!");
}

fn parsear_temp_inicial(mut temp_inicial: String) -> f32{
  temp_inicial = temp_inicial.trim().to_string();
  let inicial: f32 = temp_inicial.parse::<f32>().unwrap();
  return inicial;
}