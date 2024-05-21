use std::fs::{self, read_dir};
use std::env;
use std::io;
use std::ptr::eq;

fn rust_find(path:&str, pattern:&str) -> io::Result<()>{

    for entry in fs::read_dir(path)?{
        let entry = entry.unwrap();
        let entry_path =  entry.path();
        let entry_name = entry.file_name();
        if entry_name.to_string_lossy().eq(pattern) {
            println!("{}", entry_path.to_string_lossy());
        }

        if entry_path.is_dir() {
            _ = rust_find(&entry_path.into_os_string().into_string().unwrap(), pattern);
        }
    }
    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path: &String = &arguments[1];
    let pattern: &String = &arguments[2];
    _= rust_find(&path, &pattern);

}
