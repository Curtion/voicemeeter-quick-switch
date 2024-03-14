use serde::Deserialize;
use std::env;
use std::error::Error;
use voicemeeter::{types::Device, VoicemeeterRemote};

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
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 1 {
        panic!("Invalid arguments, please provide a status on or off");
    }
    let status = args.get(1).unwrap();
    if status == "on" {
        toggle(true).unwrap();
    } else if status == "off" {
        toggle(false).unwrap();
    } else {
        panic!("Invalid arguments, please provide a status on or off");
    }
}

fn toggle(status: bool) -> Result<(), Box<dyn Error>> {
    let config = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();
    let remote = VoicemeeterRemote::new()?;
    println!(
        "{} Verion: {}",
        remote.program,
        remote.get_voicemeeter_version()?
    );
    if !status {
        println!("Stop Device: {}", config.device);
        remote
            .parameters()
            .bus(Device::from(config.bus.clone()))?
            .device()
            .mme()
            .set("")?;
    } else {
        println!("Start Device: {}", config.device);
        remote
            .parameters()
            .bus(Device::from(config.bus.clone()))?
            .device()
            .mme()
            .set(&config.device)?;
    }
    std::thread::sleep(std::time::Duration::from_millis(config.sleep));
    Ok(())
}
