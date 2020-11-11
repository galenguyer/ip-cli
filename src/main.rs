use std::collections::HashMap;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get("https://ip.nyaa.gay/ip")?;

    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    println!("{}", body.trim());
    Ok(())
}