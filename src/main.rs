mod Functions;

use sysinfo::{System, SystemExt, CpuExt}; // Importaciones necesarias
use Functions::{procesos::obtener_top_procesos, guardar::guardar_csv};
use std::{thread, time::{Duration, Instant}, process::Command, path::Path, fs};

fn main() {
    let mut sys = System::new();
    sys.refresh_all();

    let inicio = Instant::now();
    let mut reporte_generado = false;

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_processes();

        let cpu = (
            sys.global_cpu_info().cpu_usage().clamp(0.0, 100.0),
            sys.cpus().iter().map(|c| c.cpu_usage().clamp(0.0, 100.0)).collect()
        );

        let mem = (
            sys.used_memory() / 1024,
            sys.available_memory() / 1024,
            sys.used_swap() / 1024
        );

        let procesos = obtener_top_procesos(&sys);
        guardar_csv(cpu, mem, procesos);

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