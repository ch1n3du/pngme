use std::io::{Read, Error, Write};
use std::fs::File;
use std::path::Path;

use crate::args::Commands;
use crate::chunk_type::ChunkType;
use crate::png::{Png, ChunkNotFoundError};
use crate::chunk::Chunk;

fn get_png_and_file(file_path: &String) -> Result<Png, Error> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);
    let png = Png::try_from(buf.as_ref()).unwrap();

    Ok(png)
}

fn encode(args: Commands) -> Result<(), Box<dyn std::error::Error>> {
    if let Commands::Encode { file_path, chunk_type, message, output_file} = args {
        let mut png = get_png_and_file(&file_path).unwrap();

        let new_chunk = Chunk::new(
            ChunkType::new(chunk_type.as_bytes().try_into().unwrap()), 
            message.as_bytes().iter().copied().collect()
        );
        png.append_chunk(new_chunk);
        let file = File::options().truncate(true).write(true).open(Path::new(&file_path))?;
        file.write(png.as_bytes().as_ref());

        Ok(())
    } else {
        panic!("Invalid enum variant passed as argument")
    }
}

fn decode(args: Commands) -> Result<String, ChunkNotFoundError> {
    if let Commands::Decode { file_path, chunk_type} = args {
        let png = get_png_and_file(file_path).unwrap();
        let chunk = png.chunk_by_type(chunk_type.as_str()).unwrap();

        Ok(chunk.data_as_string().unwrap())
    } else {
        panic!("Invalid Enum variant passed as argument")
    }
}


