use crate::Result;
use clap::Parser;

mod add;
mod edit;
mod list;
mod remove;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Parser)]
pub enum Command {
    Add(add::Args),
    Edit(edit::Args),
    #[clap(alias = "ls")]
    List(list::Args),
    Remove(remove::Args),
}

pub(crate) fn handle(opts: Args) -> Result<()> {
    todo!()
}
