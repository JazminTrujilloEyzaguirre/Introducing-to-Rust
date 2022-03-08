struct Rectangulo {
    ancho: u32,
    alto: u32
}

fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    // Ownership
    let rectangulo = Rectangulo { ancho:10, alto:20 };
    // el argumento rectangulo pasa como prestamo a "area", por lo tanto a no existe en "main", solo en area.
    let resultado = area(&rectangulo);
    // ampersand para que el argumento sea pasado por referencia y no por prestamo:

    println!("El área de rectangulo es: {}", resultado);

    // luego de la linea 14 ya no se puede usar rectangulo por que pertenece a "area"
    // la siguiente linea dará error si se quita el ampersand del argumento de area
    println!("El ancho y alto del rectangulo es: {} - {}", rectangulo.ancho, rectangulo.alto);
 }