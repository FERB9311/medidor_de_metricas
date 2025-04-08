mod Functions;

use Functions::{
    cpu::obtener_info_cpu,
    memoria::obtener_info_memoria,
    procesos::obtener_top_procesos,
    guardar::guardar_csv
};

use std::{thread, time::Duration};

fn main(){
    loop {
        let cpu = obtener_info_cpu();
        let mem = obtener_info_memoria();
        let procesos = obtener_top_procesos();
        guardar_csv(cpu, mem, procesos);

        thread::sleep(Duration::from_secs(10));
    }
}
