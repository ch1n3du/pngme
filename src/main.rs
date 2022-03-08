mod chunk;
mod chunk_type;
mod png;
//mod commands;
//mod args;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use chunk_type::ChunkType;

fn main() {
    let _x = ChunkType::try_from([1, 2, 3, 4]);

}
