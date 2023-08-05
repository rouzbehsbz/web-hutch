use config::Config;

#[allow(dead_code)]

mod config;
mod errors;

fn main() {
    let config = Config::from("config.yml");

    match config {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("{}", e)
    }
}