/*
Option puede trabajar con cualquier tipo de dato (T)
enum Option<T>{
    Some (T), -> El valor
    None -> La ausencía de algún valor
*/
fn obtener_valor(bandera: bool) -> Option<String> {
    if bandera {
        Some (String::from("Soy un mensaje para la tupla some!"))
    } else {
        None
    }
}
fn main() {
   // enum Option -> Si existe o no algún valor.
   // enum Result -> Errores -> panic!

    let resultado = obtener_valor(false); // Option

    // con match si existe el valor se muestra y si no, se muestra el mensaje de "no existe valor"
    //match resultado {
    //    Some(valor) => println!("El valor es: {}", valor),
    //    None => println!("No existe valor")
    //}

        // El método unwrap intenta obtener lo que la tupla Some almacena, y si no, se ejecuta un panic
    // let valor = resultado.unwrap();
    // println!("El valor es: {}", valor);

        // unwrap_or() permite mostrar un mensaje si no hay valor en la tupla
    // let valor = resultado.unwrap_or("La tupla no almacena ningun valor".to_string());
    // println!("El valor es: {}", valor);

        // expect() permite mostrar un mensaje en el panic
    let valor = resultado.expect("Se esperaba un String");
    println!("El valor es: {}", valor);
}