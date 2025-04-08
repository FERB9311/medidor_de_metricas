use sysinfo::{System, SystemExt, CpuExt, ComponentExt};

pub fn obtener_info_cpu() {
    let mut sys = System::new_all();

    // Actualizar datos del sistema
    sys.refresh_cpu();
    sys.refresh_components();

    // Info por núcleo
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("Núcleo {}:", i);
        println!("  Uso: {:.2}%", cpu.cpu_usage());
        println!("  Frecuencia: {} MHz", cpu.frequency());
    }

    // Temperaturas (puede variar según plataforma)
    println!("\nTemperaturas detectadas:");
    for componente in sys.components() {
        println!(
            "  {}: {:.1} °C",
            componente.label(),
            componente.temperature()
        );
    }
}

