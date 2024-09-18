use std::fs::OpenOptions;
use std::io::{Read, Write};

/// Reads a file and returns (bytes_read, file_content)
pub fn read_file(path: &str) -> std::io::Result<(usize, Vec<u8>)> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut content_buffer: Vec<u8> = Vec::new();
    let bytes_read = file.read_to_end(&mut content_buffer)?;
    file.flush()?;

    Ok((bytes_read, content_buffer))
}

/// Sets the file length to 0, then writes the given content to it.
pub fn clear_write_file(path: &str, new_content: Vec<u8>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open(path)?;

    file.set_len(0)?;
    file.write_all(new_content.as_ref())?;
    file.flush()?;

    Ok(())
}
