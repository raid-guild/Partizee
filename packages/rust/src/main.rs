mod partizee;
mod commands;
mod utils;
mod client;

use partizee::partizee;

pub const PROGRAM_NAME: &str = "partizee";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    partizee()
}
