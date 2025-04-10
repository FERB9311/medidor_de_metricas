use std::time::Instant;
use sysinfo::{System, SystemExt, DiskExt};
use std::io::{Write, Read};
use std::fs::File;

pub fn obtener_info_memoria() -> (u64, u64, u64) {
    let mut sys = System::new_all();
    sys.refresh_memory();
    (sys.used_memory() / 1024, sys.available_memory() / 1024, sys.used_swap() /1024)
}

pub fn get_memory() -> (u64, u64, u64, u64, u64) {
    let mut sys: System = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    (total_memory / 1024, used_memory / 1024, available_memory / 1024, total_swap / 1024, used_swap / 1024)
}

pub fn get_disk() -> Vec<(String, String, u64, u64, f64)> {
    let mut system = System::new_all();
    system.refresh_all();
    
    let mut disks_info = Vec::new();
    
    for disk in system.disks() {
        let disk_name = format!("{:?}", disk.name());
        let disk_type = format!("{:?}", disk.kind());
        let total_space = disk.total_space() / 1_000_000_000;
        let available_space = disk.available_space() / 1_000_000_000;
        
        // Medición simple de rendimiento
        let start = Instant::now();
        let mut file = std::fs::File::create("test_temp.bin").unwrap();
        let data = vec![0u8; 1024 * 1024]; // 1MB
        for _ in 0..100 {
            file.write_all(&data).unwrap();
        }
        file.sync_all().unwrap();
        let write_time = start.elapsed();
        
        let write_speed = (100.0 * 1024.0 * 1024.0) / write_time.as_secs_f64() / (1024.0 * 1024.0);
        
        disks_info.push((disk_name, disk_type, total_space, available_space, write_speed));
        
        std::fs::remove_file("test_temp.bin").unwrap();
    }
    
    disks_info
}

pub fn measure_disk_performance() -> (f64, f64, f64) {
    const FILE_PATH: &str = "disk_test.bin";
    const BLOCK_SIZE: usize = 4 * 1024; // 4KB (típico para IOPS)
    const ITERATIONS: usize = 1000;
    
    // Preparar buffer
    let write_data = vec![0u8; BLOCK_SIZE];
    
    // Medir escritura
    let mut file = File::create(FILE_PATH).unwrap();
    let write_start = Instant::now();
    for _ in 0..ITERATIONS {
        file.write_all(&write_data).unwrap();
    }
    file.sync_all().unwrap(); // Forzar escritura a disco
    let write_time = write_start.elapsed();
    
    // Medir lectura
    let mut read_data = vec![0u8; BLOCK_SIZE];
    let mut file = File::open(FILE_PATH).unwrap();
    let read_start = Instant::now();
    for _ in 0..ITERATIONS {
        file.read_exact(&mut read_data).unwrap();
    }
    let read_time = read_start.elapsed();
    
    // Calcular métricas
    let write_iops = ITERATIONS as f64 / write_time.as_secs_f64();
    let read_iops = ITERATIONS as f64 / read_time.as_secs_f64();
    let avg_latency = ((write_time + read_time).as_secs_f64() * 1000.0) / (2.0 * ITERATIONS as f64); // ms
    
    std::fs::remove_file(FILE_PATH).unwrap();
    
    (write_iops, read_iops, avg_latency)
}