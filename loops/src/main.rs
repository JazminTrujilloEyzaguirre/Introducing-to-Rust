fn main() {

    /* La palabra reservada {loop} genera un cilo por defecto infinito,
    que se rompe con la palabra reservada {break} */

    let mut contador = 0;

    loop {
        contador += 1;

        println!("Contador: {}", contador);

        if contador >= 10 {
            break;
        }
    }
}
