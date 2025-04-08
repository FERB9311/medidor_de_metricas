use sysinfo::{System, SystemExt, ProcessExt};

pub fn obtener_top_procesos() -> Vec<(String, f32)>{
    let mut sys = System::new_all();
    sys.refresh_processes();

    let mut procesos: Vec<_> = sys.processes()
        .values()
        .map(|p| (p.name().to_string(), p.cpu_usage()))
        .collect();

    procesos.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    procesos.truncate(5);

    procesos
}
