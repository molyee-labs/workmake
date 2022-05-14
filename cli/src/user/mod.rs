use crate::Result;
use clap::Parser;

mod add;
mod edit;
mod login;
mod logout;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Parser)]
pub enum Command {
    Add(add::Args),
    Login(login::Args),
    Logout(logout::Args),
    Edit(edit::Args),
    #[clap(alias = "ls")]
    List(list::Args),
}

pub(crate) fn handle(opts: Args) -> Result<()> {
    todo!()
}
