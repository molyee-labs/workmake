use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    forget: bool
}