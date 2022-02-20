use clap::Parser;
use colored::*;
use std::process::ExitCode;

mod error;
mod user;
mod company;
mod project;
mod job;
mod account;
mod search;

#[cfg(feature = "fast-alloc")]
use mimalloc_rust::GlobalMiMalloc as GlobalAlloc;

#[cfg(feature = "fast-alloc")]
#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub commands: Command
}

#[derive(Parser)]
pub enum Command {
    User(UserOpts),
    Company(CompanyOpts),
    Project(ProjectOpts),
    Job(JobOpts),
    Account(AccountOpts),
    Search(SearchOpts),
}

fn main() -> ExitCode {
    env_logger::init();
    let opts = Opts::parse();
    if let Err(e) = handle(opts) {
        print_fail()
    }
}

pub(crate) fn print_fail() {
    println!("[{}]", "FAIL".red().bold())
}

pub(crate) fn print_ok() {
    println!("[{}]", "OK".bright_blue().bold())
}

fn handle(opts: Opts) -> Result<()> {
    match opts.commands {
        Command::User(o) => user::handle(o),
        Command::Company(o) => company::handle(o),
        Command::Project(o) => project::handle(o),
        Command::Job(o) => job::handle(o),
        Command::Account(o) => account::handle(o),
        Command::Search(o) => search::handle(o),
    }
}
