use sysinfo::{System, SystemExt};
use raw_cpuid::CpuId;
use nvml_wrapper::NVML;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Retrieving system info...");
    let mut sys = System::new_all();
    sys.refresh_all();

    match (sys.long_os_version(), sys.os_version()) {
        (Some(os_name), Some(os_ver)) => println!("OS: {}, ({})", os_name, os_ver),
        (Some(os_name), None) => println!("OS: {}", os_name),
        _ => println!("Could not determine OS")
    }
    match CpuId::new().get_processor_brand_string().as_ref().map(|model| model.as_str()) {
        Some(r) => println!("Processor: {}", r),
        None => println!("Could not determine processor")
    }
    println!("Physical RAM: {} GB", ((sys.total_memory() / 1024) / 1024));
    println!("Total Memory (including Swap): {} GB", ((sys.total_swap() + sys.total_memory()) / 1024) / 1024);

    let no_gpu = || "Could not determine GPU (Not an Nvidia device, probably. ¯\\_(ツ)_/¯ )".to_string();

    match NVML::init() {
        Ok(n) => {
            let device_count = match n.device_count() {
                Ok(c) => c,
                _ => 0u32
            };
            for device in 0..device_count {
                match n.device_by_index(device) {
                    Ok(d) => println!("GPU: {}",
                                      match d.name() {
                                          Ok(gpu) => gpu,
                                          _ => no_gpu(),
                                      }
                    ),
                    _ => println!("{}", no_gpu())
                }
            }
        }
        _ => println!("{}", no_gpu())
    }

    print!("Press Enter to exit");
    let _ = stdout().flush();
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
}
