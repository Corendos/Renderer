use std::slice;
use vulkano::instance::PhysicalDevice;

pub fn print_infos(device: &PhysicalDevice) {
    println!("Device: {}", device.name());
    println!("API Version: {}", device.api_version());
    println!("Driver Version: {}", device.driver_version());
    println!("PCI Device ID: {}", device.pci_device_id());
    println!("PCI Vendor ID: {}", device.pci_vendor_id());
    println!("UUID: {}", uuid_to_string(device.uuid()));
}

fn uuid_to_string(bytes: &[u8; 16]) -> String {
    let uuid = unsafe {
        let ptr = bytes.as_ptr() as *const u32;
        slice::from_raw_parts(ptr, 4)
    };
    format!("{}-{}-{}-{}", uuid[0], uuid[1], uuid[2], uuid[3])
}
