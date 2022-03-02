fn main() {

    let mut numero: i64 = 26384625186; // cantidad de digitos : 9
    let mut contador = 0;

    while numero > 0 {
        numero = numero / 10;
        contador += 1;
    }
    println!("La cantidad de dígitos es: {}", contador);
}
