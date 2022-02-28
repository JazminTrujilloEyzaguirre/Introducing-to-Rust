fn main(){
	//operadores aritmeticos: suma, resta, division, multiplicació, modulo:
	// + - / * %
	let numero_uno: i32 = 10;
	let numero_dos: i32 = 20;
	let resultado: i32 = numero_uno % numero_dos;
	println!("Es resultado es: {}", resultado);

	// operadores relacionales
	// < > >= <= == !=
	let resultado: bool = resultado < 50;
	println!("Es resultado es: {}", resultado);

	// operadores lógicos
	// && ||
	let resultado: bool = 20 + 10 > 100 || true;
	println!("Es resultado es: {}", resultado);
}