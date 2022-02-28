fn main(){
	let mut vector = vec![1, 2, 3];

	// para agregar un elemento al final del vector
	vector.push(4);
	vector.push(5);

	println!("El valor del vector es: {:?}", vector);

	// Para agregar in valor en un indice especifico( en indice 0 agregar el valor -1)
	vector.insert(0, -1);
	vector.insert(1, 0);

	println!("El valor del vector es: {:?}", vector);

	//Para eliminar un elemento
	vector.remove(vector.len() - 1);

	vector[0] = -10;

	let primer_elemento = vector[0];
	// let ultimo elemento = vector[ vector.len() - 1]; // Esto para obtener el ultimo elemento
	// .pop para obtener el ultimo elemento y eliminarlo
	let ultimo_elemento = vector.pop().unwrap(); // se sgrega unwrap por que .pop retorna un tipo Option

	println!("El valor del vector es: {:?}", vector);

	println!("El valor del elemento es: {}", primer_elemento);
	println!("El valor del elemento es: {}", ultimo_elemento);

	println!("El valor del vector es: {:?}", vector);

    // vector con una estructura Vec::new()
    println!("Vector con una estructura a continuación");

    let mut vector = Vec::new(); // Esto creará un vector vacio.

	// Hasta que no se inserte el primer elemento no se determina el tipo de datos del vector
	vector.push(1);
	vector.push(2);
	vector.push(3);

	println!("El valor del vector es: {:?}", vector);

	let mut vector: Vec<i32> = Vec::new(); // Se puede definir el tipo de datos desde antes.
	vector.push(4);
	vector.push(5);
	vector.push(6);

	println!("El valor del vector es: {:?}", vector)
}

