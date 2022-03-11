fn main() {
    // Slices -> Heap
    // Arrays -> Stack

    let mensaje = String::from("Hola mundo, desde de Rust!");
    // para definir un slice &variablePrestada[indice inicio..indice final]
    // para obtener la palabra hola de "mensaje":
    // opcion 1:
    let hola1 = &mensaje[0..4];
    // opcion 2:
    let hola2 = &mensaje[..4];
    // para obtener el resto del mensaje:
    let resto_mensaje = &mensaje[4..];
    let mensaje_completo = &mensaje[..];

    println!("El mensaje es: {}", mensaje);
    println!("El slice es: {}", hola1);
    println!("El slice es: {}", hola2);
    println!("El slice es: {}", resto_mensaje);
    println!("El slice es: {}", mensaje_completo);
}