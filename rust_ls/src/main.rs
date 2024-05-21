use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use umask::Mode;
use users::get_user_by_uid;

fn print_entry(path:&String) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        let metadata = entry.metadata().unwrap();
        let entry_mode = Mode::try_from(path).unwrap();
        let user = get_user_by_uid(metadata.uid()).unwrap();
        println!("{:>10} {} {}", entry_mode, user.name().to_string_lossy(), path.display());
    }

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    print_entry(path);
}
