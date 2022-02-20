use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    user: Option<user::Id>,
    #[clap(short, long)]
    company: Option<company::Id>,
}