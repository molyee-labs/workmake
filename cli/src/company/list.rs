use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    user: Option<user::Id>,
}