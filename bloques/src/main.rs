fn main() {
    let mensaje = "Este es un mensaje en el bloque main";
    let mensaje_tres = "Este es un mensaje {para} el bloque anidado";

    {
        /* Con respecto a shadowing,
        aunque se defina una variable con el mismo nombre,
        se toman como variables completamente diferentes.*/
        let mensaje = "este es un mensaje en el bloque anidado";

        println!("{}", mensaje);

        let mensaje_dos = "Este es un mensaje que solo existe en el bloque anidado";

        println!("{}", mensaje_dos);

        println!("{}", mensaje_tres); // Desde los bloques anidados se puede acceder a las variables del bloque padre, pero no al revez.
    }
    println!("{}", mensaje);

    // println!("{}", mensaje_dos); // Este mensaje no podrá ser leido desde fuera del bloque anidado


    // La forma de tener acceso a una variable que existe solo en el bloque que la contiene es retornando la variable:
    let resultado = {
        println!("Segundo bloque anidado:");

        let variable: i32 = 243;

        println!("{}", variable);

        // Para retornar el valor de una variable se debe escribir su nombre al final del bloque
        variable
    };

    println!("{}", resultado);

    // Codigo que se puede refactorizar usando bloques anidados:
    {
        let calificacion: i8 = 10;
        let mut mensaje = String::new();

        if calificacion == 10{
            mensaje = String::from("aprobado :)");   // ::from || es un metodo que nos permite crear un string a partir de una cadena de caracteres
        } else {
            mensaje = String::from("Desaprobado :(");
        }

        println!("{}", mensaje);
    }

    // Bloque refactorizado:
    {
        let calificacion: i8 = 5;

        let mensaje = if calificacion == 10{
            String::from("aprobado :)")
        } else {
            String::from("Desaprobado :(")
        };

        println!("{}", mensaje);
    }
}
