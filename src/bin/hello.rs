use std::fs::{self, OpenOptions};
use std::io::{self, Write, Error};

fn main() {
    // 1. Obtenemos la ruta pidiéndosela al usuario
    let ruta: String = pedir_ruta_archivo();

    // 2. Leemos y mostramos el estado inicial del archivo
    // Pasamos "&ruta" (una referencia de lectura) para conservar la variable en main
    println!("\n📖 Contenido actual del archivo:");
    leer_y_mostrar_archivo(&ruta);

    // 3. Pedimos el nuevo texto y lo añadimos al archivo
    pedir_y_añadir_texto(&ruta);

    // 4. Volvemos a leer el archivo para ver cómo quedó finalmente
    println!("\n👀 Así ha quedado el archivo finalmente:");
    leer_y_mostrar_archivo(&ruta);
}

// -------------------------------------------------------------------------
// FUNCIONES COMPLEMENTARIAS
// -------------------------------------------------------------------------

/// Pide al usuario el nombre del archivo y lo devuelve limpio de espacios
fn pedir_ruta_archivo() -> String {
    print!("\nIntroduce el nombre o ruta del archivo (ej: poema.txt): ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    
    // Retornamos el String convertido de vuelta porque .trim() da un &str temporal
    entrada.trim().to_string()
}

/// Lee un archivo basado en la ruta provista y muestra su contenido en pantalla
/// Recibe un "&str" que acepta tanto referencias de String como texto plano
fn leer_y_mostrar_archivo(ruta_archivo: &str) {
    let resultado_lectura: Result<String, Error> = fs::read_to_string(ruta_archivo);

    match resultado_lectura {
        Ok(contenido) => {
            println!("----------------------------------------");
            println!("{contenido}");
            println!("----------------------------------------");
        }
        Err(_) => {
            println!("⚠️ El archivo no existe o está vacío.");
        }
    }
}

/// Solicita texto nuevo en la terminal y lo añade de forma segura al archivo
fn pedir_y_añadir_texto(ruta_archivo: &str) {
    // 1. Preguntar al usuario el formato deseado
    println!("\n¿Dónde deseas añadir el nuevo texto?");
    println!("1. En la misma línea (continuar párrafo)");
    println!("2. En una nueva línea (abajo)");
    print!("Selecciona una opción (1 o 2): ");
    io::stdout().flush().unwrap();

    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion).unwrap();
    let opcion = opcion.trim();

    // 2. Pedir el texto que se va a escribir
    print!("\nEscribe el texto que deseas añadir al archivo: ");
    io::stdout().flush().unwrap();

    let mut texto_nuevo = String::new();
    io::stdin().read_line(&mut texto_nuevo).unwrap();

    // 3. Verificar si el archivo ya tiene contenido previo
    let archivo_tiene_contenido = fs::read_to_string(ruta_archivo)
        .map(|c| !c.trim().is_empty())
        .unwrap_or(false);

    // 4. Preparar el archivo para escribir
    let intento_archivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(ruta_archivo);

    match intento_archivo {
        Ok(mut archivo) => {
            // 5. Aplicar el formato según la opción elegida por el usuario
            let texto_final = if archivo_tiene_contenido {
                match opcion {
                    "1" => format!(" {}", texto_nuevo), // Misma línea con espacio
                    "2" => format!("\n\n{}", texto_nuevo), // Nueva línea abajo
                    _ => {
                        println!("⚠️ Opción no válida. Se usará la opción por defecto (nueva línea).");
                        format!("\n\n{}", texto_nuevo)
                    }
                }
            } else {
                // Si el archivo está vacío, el texto va al inicio sin importar la opción
                texto_nuevo
            };

            // 6. Escribir el resultado final
            if let Err(e) = archivo.write_all(texto_final.trim_end().as_bytes()) {
                println!("❌ Error al escribir en el archivo: {e}");
            } else {
                println!("✅ ¡Texto añadido con éxito!");
            }
        }
        Err(e) => {
            println!("❌ No se pudo abrir el archivo para escribir: {e}");
        }
    }
}
