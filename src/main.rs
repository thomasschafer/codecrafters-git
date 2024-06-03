use flate2::bufread::ZlibDecoder;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};

fn init() -> io::Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");
    Ok(())
}

fn cat_file(path: &str) -> io::Result<()> {
    let file = File::open(format!(
        ".git/objects/{}/{}",
        path.chars().take(2).collect::<String>(),
        path.chars().skip(2).collect::<String>()
    ))?;
    let buf_reader = BufReader::new(file);
    let mut decoder = ZlibDecoder::new(buf_reader);

    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents)?;

    let contents_str = String::from_utf8_lossy(&contents);
    if let Some(null_pos) = contents_str.find('\0') {
        print!("{}", &contents_str[null_pos + 1..]);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Expected at least one argument");
    } else if args[1] == "init" {
        match init() {
            Ok(()) => {}
            Err(err) => println!("{}", err),
        }
    } else if args[1] == "cat-file" {
        if args.len() <= 3 || args[2] != "-p" {
            println!("Usage: git cat-file -p <path/to/file>");
            return;
        }
        match cat_file(args[3].as_str()) {
            Ok(()) => {}
            Err(err) => println!("{}", err),
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
