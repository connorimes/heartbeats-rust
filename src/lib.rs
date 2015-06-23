extern crate libc;

pub mod energy_reader;
pub mod heartbeat;

/*
use energy_reader::EnergyReader;
use heartbeat::Heartbeat;


fn main() {
    let mut er = EnergyReader::new().ok().expect("Failed to get energy reader");
    let mut hb = Heartbeat::new(None, 20, 20, "heartbeat.log", &mut er).ok().expect("Failed to get heartbeat");
    for i in 0..100 {
        hb.heartbeat(i, 1, 0.0, None);
        std::thread::sleep_ms(100);
    }
    println!("Hello, world!");
    // using Drop trait automatically cleans up
}
*/
