#![allow(unused)]
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

fn main() -> Result<(), std::io::Error> {
    // Read
    let mut f = File::open("/Users/squeeko/RustCratesLibs-Study/std-io-study/std-io-test.txt")?;
    let mut read_buffer = [0; 10]; //If the buffer goes past the character count in the file then you get zeroes
                                   // read up to 10 bytes
    f.read(&mut read_buffer); // Read returns the ASCII codes for the characters

    println!("The bytes: {:?}", read_buffer);
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
    //SeekFrom:: Start(u64), End(i64), Current(i64

    // Write
    let mut write_buffer = File::create("write_text.txt")?;
    let mut write_all_buffer = File::create("write_all_text.txt")?;

    write_buffer.write(b"some bytes are written");
    // Output is contained in file

    // Write All - This method will continuously call write until there is no more data to be written or an error of non-ErrorKind::Interrupted kind is returned.
    write_all_buffer.write_all(b"some bytes are written until buffer is exhausted");

    // Iterator using BufReader and the "lines()" function"
    let f_bufrd_iter = File::open("bufrd_iter.txt")?;
    let f_bufrd_reader = BufReader::new(f_bufrd_iter);

    for line in f_bufrd_reader.lines() {
        println!(
            "This is iterated using BufReader and the lines() function\n {:#?} ",
            line?
        );
    }

    Ok(())
}
