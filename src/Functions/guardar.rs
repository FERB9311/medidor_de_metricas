use std::fs::{OpenOptions, File};
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use chrono::Local;

pub fn guardar_csv(
    cpu: (f32, Vec<f32>), 
    mem: (u64, u64, u64), 
    procesos: Vec<(String, f32)>, 
    disk_info: Option<Vec<(String, String, u64, u64, f64)>>,
    disk_perf: Option<(f64, f64, f64)>,
    total_memory: u64,
    total_swap: u64
) {
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
        let mut encabezados = vec![
            "timestamp".into(), 
            "cpu_total".into()
        ];

        // Encabezados para los núcleos del CPU
        for i in 0..cpu_nucleos.len() {
            encabezados.push(format!("core_{}", i));
        }

        // Encabezados para memoria
        encabezados.extend([
            "mem_total_MB".into(),
            "mem_usada_MB".into(),
            "mem_libre_MB".into(),
            "swap_total_MB".into(),
            "swap_usado_MB".into()
        ]);

        // Encabezados para procesos
        for i in 1..=5 {
            encabezados.push(format!("proc_{}_nombre", i));
            encabezados.push(format!("proc_{}_cpu%", i));
        }

        // Encabezados para disco si hay datos
        if disk_info.is_some() {
            encabezados.extend([
                "disk_name".into(),
                "disk_type".into(),
                "disk_total_GB".into(),
                "disk_free_GB".into(),
                "disk_write_speed".into()
            ]);
        }

        if disk_perf.is_some() {
            encabezados.extend([
                "disk_write_iops".into(),
                "disk_read_iops".into(),
                "disk_avg_latency".into()
            ]);
        }

        writeln!(file, "{}", encabezados.join(",")).unwrap();
    }

    let mut fila = vec![
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        format!("{:.1}", cpu_total),
    ];

    // Datos del CPU
    for uso in &cpu_nucleos {
        fila.push(format!("{:.1}", uso));
    }

    // Datos de memoria
    fila.push(format!("{}", total_memory));
    fila.push(format!("{}", mem_usada));
    fila.push(format!("{}", mem_libre));
    fila.push(format!("{}", total_swap));
    fila.push(format!("{}", swap_usado));

    // Datos de procesos
    for (nombre, uso) in procesos {
        fila.push(nombre);
        fila.push(format!("{:.1}", uso));
    }

    // Datos de disco si existen
    if let Some(disks) = disk_info {
        for (name, kind, total, free, speed) in disks {
            fila.push(name);
            fila.push(kind);
            fila.push(format!("{}", total));
            fila.push(format!("{}", free));
            fila.push(format!("{:.2}", speed));
        }
    }

    if let Some((write_iops, read_iops, avg_latency)) = disk_perf {
        fila.push(format!("{:.2}", write_iops));
        fila.push(format!("{:.2}", read_iops));
        fila.push(format!("{:.4}", avg_latency));
    }

    writeln!(file, "{}", fila.join(",")).unwrap();
}

fn esta_vacio(ruta: &str) -> bool {
    File::open(ruta).map_or(true, |f| BufReader::new(f).lines().next().is_none())
}