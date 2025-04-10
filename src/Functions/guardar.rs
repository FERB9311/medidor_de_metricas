use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use chrono::Local;

pub fn guardar_csv(cpu: (f32, Vec<f32>), mem: (u64, u64, u64), procesos: Vec<(String, f32)>) {
    let (cpu_total, cpu_nucleos) = cpu;
    let (mem_usada, mem_libre, swap_usado) = mem;

    let ruta = "data/historial.csv";
    let archivo_existe = Path::new(ruta).exists();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(ruta)
        .expect("Error al abrir archivo CSV");

    if !archivo_existe || esta_vacio(ruta) {
        let mut encabezados = vec!["timestamp".into(), "cpu_total".into()];

        for i in 0..cpu_nucleos.len() {
            encabezados.push(format!("core_{}", i));
        }

        encabezados.extend([
            "mem_usada_MB".into(),
            "mem_libre_MB".into(),
            "swap_MB".into()
        ]);

        for i in 1..=5 {
            encabezados.push(format!("proc_{}_nombre", i));
            encabezados.push(format!("proc_{}_cpu%", i));
        }

        writeln!(file, "{}", encabezados.join(",")).unwrap();
    }

    let mut fila = vec![
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        format!("{:.1}", cpu_total),
    ];

    for uso in &cpu_nucleos {
        fila.push(format!("{:.1}", uso));
    }

    fila.push(format!("{:.1}", mem_usada as f64 / 1024.0));
    fila.push(format!("{:.1}", mem_libre as f64 / 1024.0));
    fila.push(format!("{:.1}", swap_usado as f64 / 1024.0));

    for (nombre, uso) in procesos {
        fila.push(nombre);
        fila.push(format!("{:.1}", uso));
    }

    writeln!(file, "{}", fila.join(",")).unwrap();
}

fn esta_vacio(ruta: &str) -> bool {
    File::open(ruta).map_or(true, |f| BufReader::new(f).lines().next().is_none())
}