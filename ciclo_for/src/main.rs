fn main() {
    let numeros: [i32; 5] = [1, 2, 3, 4, 5];

    // iterar sobre un array
    for numero in numeros.iter() {
        println!("El valor de número es: {}", numero);
    }

    // Iterar un rango
    for numero in 1..101 {
        println!("{}", numero);
    }

    // Example: Fizz Buzz
    for numero in 1..101 {
        if numero % 3 == 0 && numero % 5 == 0 {
            println!("Fizz Buzz");
        } else if numero % 3 == 0 {
            println!("Fizz");
        } else if numero % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", numero);
        }
    }
}
