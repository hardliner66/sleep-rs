struct Args {
    time: u64,
}

use clap::{App, Arg};

fn get_args() -> Result<Args, Box<dyn std::error::Error>> {
    let matches = App::new("sleep")
        .version("1.0")
        .about("Sleep for a given amount of time")
        .arg(
            Arg::with_name("time")
                .value_name("TIME")
                .help("The time to sleep in milliseconds")
                .required(true),
        )
        .get_matches();

    Ok(Args {
        time: matches.value_of("time").unwrap().parse()?,
    })
}

use std::{
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = get_args()?;
    Ok(thread::sleep(Duration::from_millis(args.time)))
}
