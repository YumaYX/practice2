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

fn main() -> io::Result<()> {
    let path = Path::new("a");

    // WRITE
    write_file(path, "Hello, world!")?;
    println!("write - success");

    // READ
    let content = read_file(path)?;
    println!("read - success {}", content);

    // DELETE
    fs::remove_file(path)?;
    println!("file del - success");

    Ok(())
}

