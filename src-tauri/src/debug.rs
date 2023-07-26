use std::env;

pub fn debug_print() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("starting dir: {}", path.display());

    Ok(())
}