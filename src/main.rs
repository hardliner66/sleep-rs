use clap::Parser;

#[derive(Parser)]
/// Sleep for a given amount of time
struct Args {
    /// The time to sleep in milliseconds
    time: u64,
}

use std::{
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    Ok(thread::sleep(Duration::from_millis(args.time)))
}
