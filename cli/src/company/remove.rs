use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(from_os_str)]
    id: company::Id,
}