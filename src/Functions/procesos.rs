use sysinfo::{System, SystemExt, ProcessExt};

pub fn obtener_top_procesos() -> Vec<(String, f32)> {
    let mut sys = System::new_all();
    sys.refresh_all(); // Actualiza todos los datos
    
    // Obtener top 5 procesos (sin filtrar por % mínimo)
    let mut procesos: Vec<_> = sys.processes()
        .values()
        .map(|p| {
            let nombre = p.name().to_string();
            let uso = p.cpu_usage(); // Uso crudo (no normalizado para ver uso real de núcleos)
            (nombre, uso)
        })
        .collect();
    
    // Ordenar descendente y tomar top 5
    procesos.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    procesos.truncate(5);
    
    procesos
}