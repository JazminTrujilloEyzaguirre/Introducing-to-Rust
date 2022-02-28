fn main(){
	// En Rust un array sigue la misma convención de indices que en la mayoría de lenguajes, comenzando en 0
							// 0  1  2  3  4
	let numeros = [1, 2, 3, 4, 5];
	println!("El valor del arreglo es: {:?}", numeros);
	// Para imprimir un array con println!, es nesesario poner en las llaves :?

	/* En Rust Los arreglos son de un solo tipo de dato,
	Se le puede definir desde antes el tipo de dato
	o dejar que Rust lo identifique, como tambien se puede definir la longitud.
	La logitud y tipo de dato no se pueden cambiar una vez definidas.*/
	let mut numeros: [i32; 6] = [1, 2, 3, 4, 5, 6]; // En caso de agregar un valor  mayor a la logitud definida [5], dará error.
	println!("El valor del arreglo es: {:?}", numeros);

	// En Rust tambbien se pueden crear arrays definiendo el valor(se repetirá) y la longitud
	let valores = [1; 10];
	println!("El valor del arreglo es: {:?}", valores);

	// Para cambiar un valor dentro del array en tiempo de ejecución, debemos vover mutable el array
	let primer_elemento = numeros[0];
	let ultimo_elemento = numeros[numeros.len() - 1]; // Para obtener el largo del array

	numeros[2] = 30;

	println!("El valor del elemento es: {}", primer_elemento);
	println!("El valor del elemento es: {}", ultimo_elemento);

	println!("El valor del arreglo es: {:?}", numeros);
}