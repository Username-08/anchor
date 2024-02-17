pub mod backends;
mod data;
mod state;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world!");
    Ok(())
}
