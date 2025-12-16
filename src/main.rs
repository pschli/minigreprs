use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        println!("{}, {}", args[1], args[2]);
    } else {
        println!("minigrep needs two arguments: <searchstring> <path/to/file.txt>")
    }
}
