use std::env;
use std::path::Path;

pub fn debug_print() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("starting dir: {}", path.display());

    Ok(())
}