mod Functions;

use Functions::{
    cpu::obtener_info_cpu,
    memoria::obtener_info_memoria,
    procesos::obtener_top_procesos,
    guardar::guardar_csv
};

use std::{
    thread,
    time::{Duration, Instant},
    process::Command,
    path::Path,
    fs,
};

//use std::{thread, time::Duration};

fn main(){
    let inicio = Instant::now();
    let mut reporte_generado_automatico = false;

    loop {
        let cpu = obtener_info_cpu();
        let mem = obtener_info_memoria();
        let procesos = obtener_top_procesos();
        guardar_csv(cpu, mem, procesos);

        thread::sleep(Duration::from_secs(30));

        if Path::new("generar_reporte.txt").exists(){
            println!("[INFO] Archivo de control detectado. Generando reporte...");
            ejecutar_reporte();
            fs::remove_file("generar_reporte.txt").ok(); //Eliminar archivo de control.
        }

        if !reporte_generado_automatico &&
            inicio.elapsed() >= Duration::from_secs(60 * 60 * 24 * 2) //2 días.
            {
                println!("[INFO] Han pasado 2 días. Generando reporte automático...");
                ejecutar_reporte();
                reporte_generado_automatico = true;
            }
    }
}

fn ejecutar_reporte(){
    let status = Command::new("python")
        .arg("scripts/generar_reporte.py")
        .status()
        .expect("Error al ejecutar el script de Python");

        if status.success(){
            println!("[OK] Reporte generado exitosamente.");
        } else{
            eprintln!("[ERROR] Falló la generación del reporte.");
        }
}