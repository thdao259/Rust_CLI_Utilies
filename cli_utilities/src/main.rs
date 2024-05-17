use std::env;

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let arg_string = arguments.join(" ");
    println!("{}", arg_string);
}
