use std::env;
use std::fs::read_dir;
use std::process::exit;

fn main() {
    let usage = "Usage: ls directory_name";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", usage);
        exit(1);
    }

    let directory = args.get(1).expect(usage);

    let dir = read_dir(directory).expect("Readdir failed");

    dir.for_each(|entry| match entry {
        Ok(value) => print!("{}\n", value.file_name().to_str().unwrap()),
        Err(_) => println!("Get file stats failed\n"),
    });
}
