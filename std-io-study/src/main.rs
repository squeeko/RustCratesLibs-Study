#![allow(unused)]
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() -> Result<(), std::io::Error> {
    let mut f = File::open("/Users/squeeko/RustCratesLibs-Study/std-io-study/std-io-test.txt")?;
    let mut buffer = [0; 10]; //If the buffer goes past the character count in the file then you get zeroes
                              // read up to 10 bytes
    f.read(&mut buffer); // Read returns the ASCII codes for the characters

    println!("The bytes: {:?}", buffer);
    /*
    Output
    The bytes: [97, 115, 100, 102, 103, 114, 100, 101, 102, 103]

    NOTE - let mut buffer = [0; 12]; //If the buffer goes past the character count in the file then you get zeroes
    Output
    The bytes: [97, 115, 100, 102, 103, 114, 100, 101, 102, 103, 0, 0]
    */

    // Seek and BufRead

    let mut f_sbr = File::open("seek_bufRead.txt")?;
    let mut buffer_sbr = [0; 10];

    f_sbr.seek(SeekFrom::End(-10));

    f_sbr.read(&mut buffer_sbr);

    println!("The bytes from the SeekFrom::End: {:?}", buffer_sbr);

    Ok(())
}
