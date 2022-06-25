use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    // Vamos a abrir el archivo en modo read only
    // para ello utilizamos el metodo open al cual
    // debemos pasarle la ruta del archivo que deseamos leer
    let archivo = File::open("poema.txt")?;

    // Ahora creamos un buffer con el archivo
    let buf_reader = BufReader::new(archivo);

    println!("Contenido del Archivo");

    // Ahora vamos a leer el archivo, pero imprimiremos
    // su contenido linea por linea.
    for linea in buf_reader.lines() {
        // Hay que notar el unwrap() para que devuelva el valor.
        // En caso este sea None, entonces el programa terminaria
        // con un error, por suerte esto lo tenemos controlado.
        println!("{}", linea.unwrap());
    }

    Ok(())
}
