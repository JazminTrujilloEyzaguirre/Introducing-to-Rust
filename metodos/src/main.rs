#[derive(Debug)]
struct User {
    username: String,
    password: String,
}
impl User {
    fn saluda(&mut self) { // self es autoreferencial
        println!("Hola, soy el usuario {}", self.username);
    }
    fn change_password (&mut self, new_password: String) {
        self.password = new_password;
    }
}

fn main() {
    let mut usuario1 = User{
        username: String::from("usuario1"),
        password: String::from("Password"),
    };

    usuario1.saluda();
    usuario1.change_password("Nuevo password".to_string());
    println!("El usuario es: {:?}", usuario1);
}

// {:?} = funciona de igual froma para vectores, tuplas, objetos, etc.
// Las Structuras no pueden usar ese formato,
// por lo que hay que agregaruna anotación que permita la lectura
// #[derive(Debug)]