use std::{fs::File, io::{self, BufRead}};

mod text_cleaner;
mod repositorio;

fn main() {
    println!("Bienvenido al contador de palabras!");
    
    let path = "data/fitness_test.txt";
    let file = File::open(path).expect("error abriendo el archivo");
    let mut buffer = io::BufReader::new(file);

    let mut linea = String::new();
    let mut linea_formateada;
    let mut repo = repositorio::Repositorio::crear_repo();
    
    while buffer.read_line(&mut linea).expect("error leyendo archivo") > 0 {
        linea_formateada = text_cleaner::formatear_linea(&linea);

        repo.guardar_palabras(&linea_formateada);
    }

    repo.imprimir_frecuencia();
}



/*
Para llenar el hashmap 
1. leo una linea
2. la separo usando split_whitespace y collect: me queda un vector de palabras
3. me fijo de limpiar la punctuation, hacer trim y pasarlas a lowercase
4. MIENTRAS voy haciendo el '3.' voy pasando las palabras al hashmap
    4.1 si la palabra esta, entonces hago un get de el value y le hago +1,
        si no esta entonces la agrego, con un 1 como value

5. sigo haciendo eso mientras que read_line() me devuelva un Ok() > 0
*/