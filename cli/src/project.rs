
use clap::Parse;

#[derive(Parse)]
pub struct ProjectOpts {
    pub commands: Command,
}

#[derive(Parse)]
pub enum Command {
    Add(AddOpts),
    Edit(EditOpts),
    Remove(RemoveOpts),
}