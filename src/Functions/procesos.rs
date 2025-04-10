use sysinfo::{System, SystemExt, ProcessExt, CpuExt}; // Importación añadida

pub fn obtener_top_procesos(sys: &System) -> Vec<(String, f32)> {
    let mut procesos: Vec<_> = sys.processes()
        .values()
        .map(|p| {
            let nombre = p.name().to_string();
            let uso = (p.cpu_usage() / sys.cpus().len() as f32).clamp(0.0, 100.0);
            (nombre, uso)
        })
        .collect();

    procesos.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    procesos.resize(5, ("N/A".to_string(), 0.0));
    procesos
}