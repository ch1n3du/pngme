use clap::StructOpt;
use pico_pngme::args::Cli;
use pico_pngme::commands::execute_command;

fn main() {
    let args = Cli::parse();
    match execute_command(args.command) {
        Ok(()) => println!("Worked successfully."),
        Err(why) => println!("{}", why),
    }
}
