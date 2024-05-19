fn main() {
    println!("Por favor introduce tu nombre: ");

    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // obtener la edad
    println!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // convertir la edad a numero
    let edad_int: u8 = edad.trim().parse().unwrap();


    println!("Hola, bienvenido/a {} de {} años", nombre, edad_int);
}
