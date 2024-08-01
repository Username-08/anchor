pub mod backends;
mod data;
pub mod handlers;
pub mod shell;
mod state;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello world!");
    for i in 0..1 {
        println!("{}", i);
    }
    Ok(())
}
