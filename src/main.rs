use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error + Sync + Send>>{
    write_hello("some_random_file")?;
    Ok(())
}

fn write_hello(file: &str) -> Result<(), Box<dyn Error + Sync + Send>> {
   File::create(file)?.write_all(b"Hello");
    Ok(())
}
