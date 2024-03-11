use std::error::Error;

use voicemeeter::{types::Device, VoicemeeterRemote};
fn main() {
    println!("Hello, world!");
    test().unwrap();
}

fn test() -> Result<(), Box<dyn Error>> {
    let remote = VoicemeeterRemote::new()?;
    println!("{}", remote.get_voicemeeter_version()?);
    println!(
        "Strip 1: {}",
        remote.parameters().strip(Device::Strip1)?.label().get()?
    );
    Ok(())
}