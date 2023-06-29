// Weather in your terminal!

// Plan
// 1. create a CLI to request weather. clap is a good option
// 2. make an HTTP request to the openweather app
// 2.1 create a .env file to store API key and token to call openweather

use clap::Parser;
use reqwest::blocking;

#[derive(Parser, Debug)]
#[command(about ="Your weather forecast", long_about = None)]
struct Args {
    /// weather function
    #[arg(short, long)]
    name: String,

    /// day for the forecast
    #[arg(short, long, default_value_t = 0)]
    day: u8,
}

fn main() {
    let args = Args::parse();


    let body = reqwest::blocking::get("https://www.rust-lang.org").unwrap()
    .text().unwrap();

    println!("body = {:?}", body);
}