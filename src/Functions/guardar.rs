use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;

pub fn guardar_csv(cpu: (f32, Vec<f32>), mem: (u64, u64), procesos: Vec<(String, f32)>) {
    let _ = create_dir_all("data"); // Crea la carpeta si no existe

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let line = format!("{},{:.2},{},{},{}",
        timestamp,
        cpu.0,
        mem.0,
        mem.1,
        procesos
            .into_iter()
            .flat_map(|(n, c)| vec![n, format!("{:.2}", c)])
            .collect::<Vec<_>>()
            .join(",")
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/historial.csv")
        .unwrap();

    writeln!(file, "{}", line).unwrap();
}
