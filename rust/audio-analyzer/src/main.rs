use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("src/assets/test.mp3");
    let file = File::open(file_path)?;

    let metadata = file.metadata();


    if metadata.is_ok() {
        let file_info = metadata.unwrap();
        let file_size = file_info.len();

        let file_type = file_info.file_type();

        read_file_contents(file)?;
    } else {
        println!("Something wrong happened")
    }


    Ok(())
}

fn read_file_contents(file: File) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(file);


    let mut bytes = Vec::new();


    let mut chunk = [0; 8192];
    while let Ok(bytes_read) = buf_reader.read(&mut chunk) {
        if bytes_read == 0 {
            break;
        }

        bytes.extend_from_slice(&chunk[..bytes_read]);
    }

    let reconstructed_bytes = reconstruct_file(bytes).ok();

    match reconstructed_bytes {
        Some(bytes) => {
            println!("Reconstructed bytes: {:?}", bytes);

            let response = write_file(bytes);
            match response {
                Ok(_) => {
                    println!("File written successfully");
                }
                Err(_) => {
                    println!("Error writing file");
                }
            }
        }
        _ => {
            println!("Error reconstructing bytes");
        }
    }

    Ok(())
}


fn reconstruct_file(bytes: Vec<u8>) -> Result<Vec<u8>, ()> {
    if bytes.is_empty() {
        println!("No bytes to reconstruct");
        return Ok(Vec::new());
    }

    println!("Reconstructing file");

    if bytes.len() % 2 != 0 {
        return Ok(bytes);
    }


    if bytes.len() == 2 {
        return Ok(vec![bytes[1], bytes[0]]);
    }


    let new_length = bytes.len() / 2;
    let left_bytes = &bytes[0..new_length];
    let right_bytes = &bytes[new_length..];


    let mut reconstructed_bytes = Vec::new();
    let left_result = reconstruct_file(left_bytes.to_vec())?;
    let right_result = reconstruct_file(right_bytes.to_vec())?;

    reconstructed_bytes.extend_from_slice(&right_result);
    reconstructed_bytes.extend_from_slice(&left_result);

    Ok(reconstructed_bytes)
}

fn write_file(bytes: Vec<u8>) -> std::io::Result<()> {
    let slice_bytes: &[u8] = bytes.as_slice();

    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join("src/assets/");

    let file_name = "assets/export.mp3";

    let combined_path = file_path.with_file_name(file_name);
    let mut file = File::create(combined_path)?;
    file.write_all(slice_bytes)?;
    Ok(())
}

