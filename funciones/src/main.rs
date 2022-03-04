fn saludar_usuario() {
    println!("Hola, desde una funcion");
}

fn suma(numero_uno: i32, numero_dos: i32) -> i32 {
    numero_uno + numero_dos
}

fn factorial(numero: u32) -> u32{
    if numero == 1 {
        return numero;
    }

    factorial(numero - 1) * numero
}

fn main() {
    saludar_usuario();
    let resultado = suma(10, 20);
    println!("El resultado es: {}", resultado);
    let resultado = factorial(5);
    println!("El factorial es: {}", resultado);
}
