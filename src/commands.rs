use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

use crate::args::CliCommand;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::{ChunkNotFoundError, Png};

fn get_png(file_path: &String) -> Result<Png, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf);
    let png = Png::try_from(buf.as_ref())?;

    Ok(png)
}

fn overwrite_file(file_path: &String, buf: &[u8]) -> Result<(), Error> {
    let mut file = File::options()
        .truncate(true)
        .write(true)
        .open(Path::new(file_path))?;
    file.write(buf);

    Ok(())
}

fn encode(command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
    if let CliCommand::Encode {
        file_path,
        chunk_type,
        message,
        output_file,
    } = command
    {
        let mut png = get_png(&file_path).unwrap();

        let new_chunk = Chunk::new(
            ChunkType::new(chunk_type.as_bytes().try_into().unwrap()),
            message.as_bytes().iter().copied().collect(),
        );
        png.append_chunk(new_chunk);
        let buf = png.as_bytes();

        match output_file {
            None => overwrite_file(&file_path, buf.as_ref())?,
            Some(x) => overwrite_file(&x, buf.as_ref())?,
        }

        return Ok(());
    }

    panic!("Invalid enum variant passed as argument")
}

fn decode(command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
    if let CliCommand::Decode {
        file_path,
        chunk_type,
    } = command
    {
        let png = get_png(&file_path).unwrap();
        let chunk = png.chunk_by_type(chunk_type.as_str()).unwrap();

        println!("{}", chunk.data_as_string().unwrap());
        return Ok(());
    }

    panic!("Invalid Enum variant passed as argument")
}

fn remove(command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
    if let CliCommand::Remove {
        file_path,
        chunk_type,
    } = command
    {
        let mut png = get_png(&file_path)?;
        png.remove_chunk(chunk_type.as_str());
        overwrite_file(&file_path, png.as_bytes().as_ref());

        return Ok(());
    }

    panic!("Invalid Enum variant passed as argument")
}

fn print(command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
    if let CliCommand::Print { file_path } = command {
        let png = get_png(&file_path)?;
        println!("{}", png.to_string());

        return Ok(());
    }

    panic!("Invalid Enum variant passed as argument")
}

pub fn execute_command(command: CliCommand) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        CliCommand::Decode { .. } => decode(command)?,
        CliCommand::Encode { .. } => encode(command)?,
        CliCommand::Remove { .. } => remove(command)?,
        CliCommand::Print { .. } => print(command)?,
    };

    Ok(())
}
