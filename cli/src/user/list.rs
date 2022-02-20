use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long, default_value = 30)]
    limit: u16,
    #[clap(short, long)]
    company: Option<company::Id>,
}