// mod chunk;
// mod chunk_type;
// mod png;
// //mod commands;
// //mod args;

// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

// use chunk_type::ChunkType;

// fn main() {
//     let _x = ChunkType::try_from([1, 2, 3, 4]);

// }

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}