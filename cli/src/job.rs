use clap::Parse;

#[derive(Parse)]
pub struct JobOpts {
    pub commands: Command,
}

#[derive(Parse)]
pub enum Command {
    Add(AddOpts),
    List(ListOpts),
    Remove(RemoveOpts),
}
