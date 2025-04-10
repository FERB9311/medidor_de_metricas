use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use chrono::Local;

pub fn guardar_csv(
    cpu: (f32, Vec<f32>),
    mem: (u64, u64, u64), // Incluimos el valor de swap aquí
    procesos: Vec<(String, f32)>
) {
    let (cpu_total, cpu_nucleos) = cpu;
    let (mem_usada, mem_libre, swap_usado) = mem; // Desempaquetamos la memoria usada, libre y swap usado

    let ruta = "data/historial.csv";
    let archivo_existe = Path::new(ruta).exists();

    // Abrir el archivo para agregar datos
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(ruta)
        .expect("No se pudo abrir o crear el archivo CSV");

    // Si el archivo es nuevo o está vacío, escribir encabezados
    if !archivo_existe || esta_vacio(ruta) {
        let mut encabezado = vec![
            String::from("timestamp"),
            String::from("cpu_total"),
        ];

        // Encabezados para núcleos
        for i in 0..cpu_nucleos.len() {
            encabezado.push(format!("core_{}", i));
        }

        encabezado.push("mem_usada_MB".to_string());
        encabezado.push("mem_libre_MB".to_string());
        encabezado.push("swap_usado_MB".to_string()); // Agregar swap a los encabezados

        // Encabezados para procesos (máximo 5)
        for i in 0..procesos.len() {
            encabezado.push(format!("proceso_{}", i + 1));
            encabezado.push(format!("cpu_uso_del_proceso{}", i + 1));
        }
        
        writeln!(file, "{}", encabezado.join(",")).unwrap();
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Comenzamos con la hora y el CPU total
    let mut fila = vec![
        timestamp,
        format!("{:.2}", cpu_total),
    ];

    // Agregar uso por núcleo
    for uso in cpu_nucleos {
        fila.push(format!("{:.2}", uso));
    }

    // Agregar memoria
    fila.push(format!("{:.2}", mem_usada as f64 / 1024.0)); // Usada en MB
    fila.push(format!("{:.2}", mem_libre as f64 / 1024.0)); // Libre en MB
    fila.push(format!("{:.2}", swap_usado as f64 / 1024.0)); // Swap usado en MB

    // Agregar procesos
    for (nombre, uso) in procesos.iter().take(5) { // Limitar a los 5 procesos más altos
        fila.push(nombre.clone());
        fila.push(format!("{:.2}", uso));
    }

    writeln!(file, "{}", fila.join(",")).expect("Error al escribir en el archivo CSV");
}

// Función auxiliar para verificar si el archivo está vacío
fn esta_vacio(ruta: &str) -> bool {
    if let Ok(file) = File::open(ruta) {
        let mut reader = BufReader::new(file);
        let mut linea = String::new();
        reader.read_line(&mut linea).ok().map_or(true, |n| n == 0)
    } else {
        true
    }
}
