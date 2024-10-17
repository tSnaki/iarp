use std::{fs::File, io::{BufRead, BufReader}};

/// Reads the file using buffered reader.
/// May use threaded reading in the future.
/// Returns the content of the file in a string format.
fn read_file(path: String) -> std::io::Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    return Result::Ok(line);
}