use pcap;
use std::collections::HashMap;

fn main() {
    // Create a map where we store available devices.
    let mut devices_hash_set: HashMap<String, pcap::Device> = HashMap::new();

    println!("device:\t description");
    for device in pcap::Device::list().expect("Failed to list devices").iter() {
        // Insert each devices into a hashmap.
        devices_hash_set.insert(device.name.clone(), device.clone());
        println!(
            "{}: {}",
            device.name,
            device
                .desc
                .clone()
                .unwrap_or("description not available".to_string())
        );
    }

    //loop {
    //    match devices_hash_set.get(&"en0".to_string()) {
    //        Some(device) => {
    //            device
    //                .clone()
    //                .open()
    //                .unwrap()
    //                .next_packet()
    //                .unwrap()
    //                .to_vec()
    //                .iter()
    //                .for_each(|elem| println!("{}", elem.to_ascii_lowercase().to_string()));
    //        }
    //        None => println!("en0 device not found"),
    //    }
    //}
}
