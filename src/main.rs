use clap::StructOpt;
use pngme::args::Cli;
use pngme::commands::execute_command;

fn main() {
    let args = Cli::parse();
    match execute_command(args.command) {
        Ok(()) => println!("Worked successfully."),
        Err(why) => println!("{}", why),
    }
}
