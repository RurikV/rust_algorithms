use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a filename is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Get the filename from the arguments
    let file_path = &args[1];

    // Open the file
    let path = Path::new(file_path);
    let mut file = File::open(&path)?;

    // Read the file contents into a vector of bytes
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Print the bytes in the specified format
    print!("[");
    for (i, byte) in buffer.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("0x{:02X}", byte);
    }
    println!("]");

    Ok(())
}
