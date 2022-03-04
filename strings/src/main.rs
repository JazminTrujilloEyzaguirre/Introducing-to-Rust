fn main() {
    // str -> Stack
    // String -> Heap

    //str
    let variable_str = "Soy un tipo str";
    // String new para un string vacio
    let string_vacio = String::new();
    // String from para convertir un tipo str a String
    let mut string_from = String::from("Hola, soy un String");

    // Para agregar un caracter a un string
    string_from.push('.');
    // Para agregar un str a un string
    string_from.push_str("Curso Rust");

    let nuevo_string = "Soy una cadena".to_string(); // convertir un srt a String

    // Operadores con strings
    let igual = nuevo_string == "Soy una cadena diferente".to_string();
    let diferente = nuevo_string != "Soy una cadena diferente".to_string();

    println!("El srt es: {}", variable_str);
    println!("El String es: {}", string_vacio);
    println!("El String es: {}", string_from);
    println!("El String es: {}", nuevo_string);
    println!("¿Los string son iguales?: {}", igual);
    println!("¿Los string son diferentes?: {}", diferente);
}
