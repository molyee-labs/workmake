use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    company: company::Id,
    #[clap(from_os_str)]
    name: String,
}