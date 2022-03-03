fn main() {
    // la convensión es nombrar enums con CamelCase
    enum Response { // enum puede contener arrays, tuplas, vectores, etc
        Success,
        Error(i32, String), // 403, 404, 500
    }

    let respuesta = Response::Error(501, String::from("No es posible completar la operación."));

    match respuesta {
        Response::Success => println!("La petición fue exitosa."),
        Response::Error(403, _) =>  println!("Forbidden"),
        Response::Error(404, _) =>  println!("Not Found"),
        Response::Error(500, _) =>  println!("Internal server error"),
        Response::Error(_, mensaje) =>  println!("{}", mensaje)
    }
}
