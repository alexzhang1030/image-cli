use clap::{ Parser, Subcommand };
use commands::resize::{ ResizeArgs, handler_resize };

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Resize an image")] Resize(ResizeArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Resize(args) => { handler_resize(args) }
    }
}