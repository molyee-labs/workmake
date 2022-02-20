use clap::Parse;

#[derive(Parse)]
pub struct UserOpts {
    pub commands: Command,
}

#[derive(Parse)]
pub enum Command {
    Add(AddOpts),
    Login(LoginOpts),
    Logout(LogoutOpts),
    Edit(EditOpts),
}