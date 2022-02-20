use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    user: Login,
    #[clap(short, long)]
    pass: Password,
}