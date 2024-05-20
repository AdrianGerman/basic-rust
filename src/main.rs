fn main() {
    // declaramos los números
    let numero_1 = 123;
    let numero_2 = 321;

    let suma = numero_1 + numero_2;

    loop {
        // mostrar los 2 números en pantalla
        println!("Por favor ingrese la suma de {} y {}: ", numero_1, numero_2);

        // obtener del usuario el numero que representa la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Muy bien el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} es incorrecto, ni para eso sirves", suma_usuario_int);
            println!();
        }
    }
}
