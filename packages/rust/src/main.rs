mod client;
mod commands;
mod partizee;
mod utils;

use partizee::partizee;

pub const PROGRAM_NAME: &str = "partizee";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    partizee()
}
