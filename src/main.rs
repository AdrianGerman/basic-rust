fn main() {
    // obtener el nombre del usuario
    println!("Por favor introduce tu nombre: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // obtener el país del usuario
    println!("Por favor introduce tu país: ");
    let mut pais: String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap();
    nombre = nombre.trim().to_string();

    println!("Hola, bienvenido/a {} de {}", nombre, pais);
}
