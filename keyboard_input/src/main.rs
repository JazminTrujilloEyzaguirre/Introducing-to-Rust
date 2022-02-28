use std::io;

fn main(){
	println!("Ingresa el nombre de usuario");
	let mut username = String::new(); // Static -> "" metodo estático que retorna un string vacio
	io::stdin().read_line(&mut username); // referencia
	// la funcion read_line recibe como parametro un string
	// y lo  modifica para otorgarle el valor del input que escribirá el usuario
	// al poner el uppersone con mut, le damos permiso para modificar la variable
	let username = username.trim(); // el método trim elimina los saltos de linea
	println!("El nombre de usuario es: {}", username);
}