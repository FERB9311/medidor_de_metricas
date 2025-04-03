use sysinfo::{System, SystemExt, CpuExt};

pub fn obtener_metricas_basicas() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_uso = sys.global_cpu_info().cpu_usage();
    let memoria_uso = sys.used_memory() / 1024;

    println!("CPU: {}%", cpu_uso);
    println!("Memoria usada: {} MB", memoria_uso);
}
