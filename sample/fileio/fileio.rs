use std::{
    fs,
    io,
    path::Path,
};

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

fn main() {
    let path = Path::new("a");

    // WRITE
    let wf = write_file(path, "Hello, world!");
    match wf {
        Ok(()) => println!("write - success"),
        Err(e) => eprintln!("write - fail: {}", e),
    }

    // READ
    let rf = read_file(path);
    match rf {
        Ok(content) => println!("read - success {}",content),
        Err(err) => eprintln!("read - fail {}",err),
    }

    if let Err(e) = fs::remove_file(path) {
        eprintln!("file del - fail: {}", e);
    } else {
        println!("file del - success");
    }
}

