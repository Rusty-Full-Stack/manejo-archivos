use std::fs::File;
// Agregando el modulo de Buffer para lectura
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    // Vamos a abrir el archivo en modo read only
    // para ello utilizamos el metodo open al cual
    // debemos pasarle la ruta del archivo que deseamos leer
    let archivo = File::open("poema.txt")?;

    // Ahora creamos un buffer con el archivo, la ventaja de hacerlo
    // asi es que no debemos definir nuestro File como mutable, por
    // lo que es mas eficiente.
    let mut buf_reader = BufReader::new(archivo);
    let mut contenido = String::new();

    // Esta linea se encarga de leer el contenido completo
    // del archivo y lo coloca en un String, en este caso es
    // la mut variable contenido, el metodo read_to_string
    // recibe de parametro la referencia de la variable String
    // a la cual queremos pasarle el contenido del archivo
    buf_reader.read_to_string(&mut contenido)?;

    println!("Contenido del Archivo");
    println!("{}", contenido);

    Ok(())
}
