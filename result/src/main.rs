/*
enum ResultkT, E>{
    Ok (T), I
    Err(E)
}
*/
// Result tiene 3 métodos que intentan obtener lo que "Ok" almacena.
// los métodos son: unwrap, unwrap_or y expect

enum ErrorDivision {
    DivisionPorCero,
    DivisionNegativos
}

fn division(numero1: i32, numero2: i32) -> Result<i32, ErrorDivision> {
    if numero2 == 0 {
       return Err(ErrorDivision::DivisionPorCero);
    }

    if numero2 == 0 || numero2 < 0 {
        return Err(ErrorDivision::DivisionNegativos);
    }

    Ok (numero1 / numero2)
 }
fn main() {
    // Result
    let resultado = division(-10, -5); // Result
    match resultado {
        Ok (valor) => println!("El resultado es: {}", valor),
        Err(ErrorDivision::DivisionPorCero) => {
            println!("El error es por intentar dividir numeros entre 0");
        }
        Err(ErrorDivision::DivisionNegativos) => {
            panic!("El error es por intentar dividir numeros negativos");
        }
    }
}