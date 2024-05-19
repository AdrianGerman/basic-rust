fn main() {
    // obtener el nombre del usuario
    println!("Por favor introduce tu edad: ");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // convertir la edad a enteros
    let edad_int: u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int != 30 {
        println!("Puedas entrar a la discoteca");
    } else if edad_int == 30 {
        println!("No admitimos personas exactamente de 30 años");
    } else {
        println!("Eres menos de edad, todavía no puedes entrar");
    }

    // if edad_int >= 18 {
    //     println!("Puedes entrar a la discoteca");
    // } else {
    //     println!("No puedes entrar a la discoteca")
    // }

    println!("Tienes {} años", edad_int);

}
