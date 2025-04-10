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
    let mut disk_check_counter = 0;

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_processes();

        let cpu = (
            sys.global_cpu_info().cpu_usage().clamp(0.0, 100.0),
            sys.cpus().iter().map(|c| c.cpu_usage().clamp(0.0, 100.0)).collect()
        );

        // Obtenemos la información de memoria de ambas formas para compatibilidad
        let mem_simple = obtener_info_memoria();
        let (total_memory, used_memory, available_memory, total_swap, used_swap) = get_memory();
        
        // Usamos la información detallada de memoria
        let mem = (
            used_memory,    // mem_usada en MB
            available_memory, // mem_libre en MB
            used_swap       // swap_usado en MB
        );

        let procesos = obtener_top_procesos(&sys);
        
        // Solo verificar disco cada 10 iteraciones (5 minutos)
        let disk_info = if disk_check_counter % 10 == 0 {
            Some(get_disk())
        } else {
            None
        };

        let disk_perf = if disk_check_counter % 30 == 0 { // Cada 15 minutos
            Some(measure_disk_performance())
        } else {
            None
        };

        guardar_csv(
            cpu, 
            mem, 
            procesos, 
            disk_info, 
            disk_perf,
            total_memory,  // Pasamos la memoria total adicional
            total_swap     // Pasamos el swap total adicional
        );
        
        disk_check_counter += 1;

        if Path::new("generar_reporte.txt").exists() {
            ejecutar_reporte();
            fs::remove_file("generar_reporte.txt").unwrap();
        }

        if !reporte_generado && inicio.elapsed() >= Duration::from_secs(172800) {
            ejecutar_reporte();
            reporte_generado = true;
        }

        thread::sleep(Duration::from_secs(30));
    }
}

fn ejecutar_reporte() {
    Command::new("python")
        .arg("scripts/generar_reporte.py")
        .status()
        .expect("Error al ejecutar script Python");
}