use sysinfo::{System, SystemExt, CpuExt};

pub fn obtener_info_cpu() -> (f32, Vec<f32>) {
    let mut sys = System::new_all();
    sys.refresh_cpu();

    let total = sys.global_cpu_info().cpu_usage();
    let por_nucleo = sys.cpus().iter().map(|c| c.cpu_usage()).collect();

    (total, por_nucleo)
}
