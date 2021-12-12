use std::{fs::File, io::Write};

fn main() {
    write_hello("some_random_file")
}

fn write_hello(file: &str) {
    let r  = File::create(file);

    if let Err(e) = r {
        println!("hey there is some error creating file: {:?}", e);
        return;
    }
    let mut f  = r.unwrap();

    let r = f.write_all(b"Hello");

    if let Err(e) = r {
        println!("hey there is some error {:?}", e);
    }
}
