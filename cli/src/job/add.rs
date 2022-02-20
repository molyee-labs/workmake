use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    project: Option<project::Id>,
    #[clap(short, long)]
    desc: Option<String>,
    #[clap(from_os_str)]
    name: String,
}