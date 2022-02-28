fn main(){
	let tupla = (10, false, 5.5);
	println!("El valor de la tupla es: {:?}", tupla);

	let mut tupla: (i32, bool, f64, i32) = (10, false, 5.5, 99);
	println!("El valor de la tupla es: {:?}", tupla);

	// Obtener indices de una tupla
	let primer_elemento = tupla.0;
	let ultimo_elemento = tupla.3;

	// Cambiar un valor de la tupla (siempre que sea mutable)
	tupla.1 = true;

	println!("El elemento es: {}", primer_elemento);
	println!("El elemento es: {}", ultimo_elemento);

	println!("El valor del tupla es: {:?}", tupla);
}
// Las tuplas son similares a los arrays, pero en estas se pueden alamacenar diferentes tipos de datos.