use clap::Parse;

#[derive(Parse)]
pub struct AccountOpts {
    pub commands: Command,
}

#[derive(Parse)]
pub enum Command {
}
