fn main(){
	let valor: i32 = 10; // inmutable
	println!("El valor de la variable es: {}", valor);
	let valor = 20;
	println!("El valor de la variable es: {}", valor);
	let valor = true; // El tipo de dato de la sombra puede ser diferente al tipo de dato de la variable original.
	println!("El valor de la variable es: {}", valor);

    /* La primera variable spaces es un tipo string y la segunda variable space es un tipo de número. 
    El sombreado nos evita tener que inventar diferentes nombres, como spaces_stry spaces_num. */

    let spaces = "   ";
    let spaces = spaces.len();
	println!("El numero de espacios es: {}", spaces);
}

// En Rust existe el término shadowing, que dice que podemos utilizar el nombre de una variable una $n$ cantidad de veces.