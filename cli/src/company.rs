use clap::Parse;

#[derive(Parse)]
pub struct CompanyOpts {
    pub commands: Command,
}

#[derive(Parse)]
pub enum Command {
    Add(AddOpts),
    Edit(EditOpts),
}
