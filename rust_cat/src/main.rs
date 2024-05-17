use std::env;
use std::io;
use std::fs::File;
use std::io::Read;

fn cat_file(file_name: &str) -> Result<String, io::Error> {

    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return Err(e)
        },
    };

    let mut content = String::new();
    username_file.read_to_string(&mut content)?;
    println!("{}", content);
    Ok(content)
}
fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    let file_path:&str = &args[0];
    println!("{}", file_path);
    _ = cat_file(file_path);
}
