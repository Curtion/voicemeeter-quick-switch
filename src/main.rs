use std::error::Error;
use voicemeeter::{types::Device, VoicemeeterRemote};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    bus: Bus,
    device: String,
    sleep: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Bus {
    String(String),
}

impl From<Bus> for Device {
    fn from(device: Bus) -> Self {
        match device {
            Bus::String(device) => match device.as_str() {
                "OutputA1" => Device::OutputA1,
                "OutputA2" => Device::OutputA2,
                "OutputA3" => Device::OutputA3,
                "OutputA4" => Device::OutputA4,
                "OutputA5" => Device::OutputA5,
                _ => panic!("Invalid device"),
            },
        }
    }
}

fn main() {
    let config = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();
    toggle(config.clone()).unwrap();
}

fn toggle(config: Config) -> Result<(), Box<dyn Error>> {
    // println!(
    //     "{} Verion: {}",
    //     remote.program,
    //     remote.get_voicemeeter_version()?
    // );
    let remote = VoicemeeterRemote::new()?;
    loop {
        let remote = VoicemeeterRemote::new()?;
        let current_device = get_current_name(remote, config.clone())?;
        println!("Current Device: {}", current_device);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    // remote
    //     .parameters()
    //     .bus(Device::from(config.bus.clone()))?
    //     .device()
    //     .mme()
    //     .set("")?;
    // remote
    //     .parameters()
    //     .bus(Device::from(config.bus.clone()))?
    //     .device()
    //     .mme()
    //     .set(&config.device)?;
    // let current_device = remote
    //     .parameters()
    //     .bus(Device::from(config.bus.clone()))?
    //     .device()
    //     .name()
    //     .get()?;
    // if current_device == config.device {
    //     println!("Stop Device: {}", current_device);
    //     remote
    //         .parameters()
    //         .bus(Device::from(config.bus.clone()))?
    //         .device()
    //         .mme()
    //         .set("")?;
    // } else {
    //     println!("Start Device: {}", config.device);
    //     remote
    //         .parameters()
    //         .bus(Device::from(config.bus.clone()))?
    //         .device()
    //         .mme()
    //         .set(&config.device)?;
    // }
    Ok(())
}

fn get_current_name(remote: VoicemeeterRemote, config: Config) -> Result<String, Box<dyn Error>> {
    let current_device = remote
        .parameters()
        .bus(Device::from(config.bus.clone()))?
        .device()
        .name()
        .get()?;
    Ok(current_device)
}