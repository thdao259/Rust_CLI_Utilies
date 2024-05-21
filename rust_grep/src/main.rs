use std::env;
use std::fs;
use std::io;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn rust_grep(path:&str, pattern:&str) -> Result<()>{
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path_str = entry.path().into_os_string().into_string().unwrap();
        if entry.metadata()?.is_file() {
            let lines = read_lines(entry.path())?;
            for (i, line) in lines.flatten().enumerate() {
                let found: Vec<&str> = line.matches(pattern).collect();
                if found.len() > 0 {
                    println!("{}:{}: {}", entry_path_str, i+1, line);
                }
            }
        } else {
            _ = rust_grep(&entry_path_str, pattern);
        }
    }

    Ok(())

}
fn main() {
    let arguments: Vec<String> = env::args().collect();
    _ = rust_grep(&arguments[1], &arguments[2]);
}
