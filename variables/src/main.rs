fn main(){
	let numero_uno = 10; // inmutable
	let mut numero_dos = 20; // mutable
	let numero_tres: i32 = 30; // tipo de dato definido
	numero_dos = 40;
	let resultado = numero_uno + numero_dos + numero_tres;
	println!("El resultado de la suma ({}, {}, {}) es: {}", numero_uno, numero_dos, numero_tres, resultado);
}