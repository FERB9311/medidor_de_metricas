use sysinfo::{System, SystemExt};

pub fn obtener_info_memoria() -> (u64, u64, u64){
    let mut sys = System::new_all();

    sys.refresh_memory();

    (sys.used_memory() / 1024, sys.available_memory() / 1024, sys.used_swap() /1024)
}