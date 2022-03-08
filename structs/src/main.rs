struct User {
    username: String,
    password: String
}

fn create_user(username: String, password: String) -> User {
    User { username, password}
}

fn main() {
    let username = String::from("Parsiomnium");
    let password = String::from("password123");

    let mut usuario = create_user(username, password);

    println!("El username es: {}", usuario.username);
    usuario.username = String::from("Cambio de valor");

    println!("El username es: {}", usuario.username);
    println!("El password es: {}", usuario.password);
}
