mod Functions;

use sysinfo::{System, SystemExt, CpuExt};
use Functions::{
    procesos::obtener_top_procesos, 
    guardar::guardar_csv, 
    memoria::{get_memory, get_disk, measure_disk_performance, obtener_info_memoria}
};
use std::{thread, time::{Duration, Instant}, process::Command, path::Path, fs};

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    let inicio = Instant::now();
    let mut reporte_generado = false;

    loop {
        // Refrescar datos del sistema cada 5 minutos
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_processes();

        let cpu = (
            sys.global_cpu_info().cpu_usage().clamp(0.0, 100.0),
            sys.cpus().iter().map(|c| c.cpu_usage().clamp(0.0, 100.0)).collect()
        );

        // Obtener información de memoria
        let mem_simple = obtener_info_memoria();
        let (total_memory, used_memory, available_memory, total_swap, used_swap) = get_memory();
        
        let mem = (
            used_memory,    // mem_usada en MB
            available_memory, // mem_libre en MB
            used_swap       // swap_usado en MB
        );

        let procesos = obtener_top_procesos(&sys);
        let disk_info = get_disk();
        let disk_perf = measure_disk_performance();

        guardar_csv(
            cpu, 
            mem, 
            procesos, 
            Some(disk_info), 
            Some(disk_perf),
            total_memory,
            total_swap
        );

        // Verificar si se debe generar el reporte manual cada 5 minutos
        if Path::new("generar_reporte.txt").exists() {
            ejecutar_reporte();
            fs::remove_file("generar_reporte.txt").unwrap();
        }

        // Reporte automático después de 48 horas (solo una vez)
        if !reporte_generado && inicio.elapsed() >= Duration::from_secs(172800) {
            ejecutar_reporte();
            reporte_generado = true;
        }

        // Esperar 5 minutos (300 segundos) antes de la próxima iteración
        thread::sleep(Duration::from_secs(300));
    }
}

fn ejecutar_reporte() {
    // Copiar el archivo CSV actual a un lugar donde Python pueda acceder
    let _ = std::fs::copy("data/historial.csv", "historial_temp.csv");
    
    Command::new("python")
        .arg("scripts/generar_reporte.py")
        .status()
        .expect("Error al ejecutar script Python");
    
    // Limpiar archivo temporal
    let _ = std::fs::remove_file("historial_temp.csv");
}