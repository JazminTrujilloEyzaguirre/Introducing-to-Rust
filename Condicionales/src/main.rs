use std::io;

fn main(){
	let mut color = String::new();

	println!("Ingresa un color para el semaforo:");

	io::stdin().read_line(&mut color);

	let color = color.trim().to_lowercase(); // Para eliminar salto de linea y estandarizar mayus y minus.

	if color == "verde" {
		println!("Puede continuar");
	} else if  color == "amarillo" {
		println!("Alto parcial.");
	} else if color == "rojo" {
		println!("Alto total");
	} else {
		println!("El color no es valido");
	}

}