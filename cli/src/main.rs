#![feature(process_exitcode_placeholder)]

mod error;
mod user;
mod company;
mod project;
mod job;
mod account;
mod search;

use clap::Parser;
use colored::*;
use std::process::ExitCode;

pub use crate::error::Error;
pub(crate) use crate::error::Result;

#[cfg(feature = "fast-alloc")]
use mimalloc_rust::GlobalMiMalloc as GlobalAlloc;

#[cfg(feature = "fast-alloc")]
#[global_allocator]
static GLOBAL: GlobalAlloc = GlobalAlloc;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub commands: Command
}

#[derive(Parser)]
pub enum Command {
    User(user::Args),
    Company(company::Args),
    Project(project::Args),
    Job(job::Args),
    Account(account::Args),
    Search(search::Args),
}

fn main() -> ExitCode {
    env_logger::init();
    let opts = Args::parse();
    if let Err(e) = handle(opts) {
        print_fail();
        eprintln!("{}: {}", "Error".bright_red().bold(), &e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

pub(crate) fn print_fail() {
    println!("[{}]", "FAIL".red().bold())
}

pub(crate) fn print_ok() {
    println!("[{}]", "OK".bright_blue().bold())
}

fn handle(opts: Args) -> Result<()> {
    match opts.commands {
        Command::User(o) => user::handle(o),
        Command::Company(o) => company::handle(o),
        Command::Project(o) => project::handle(o),
        Command::Job(o) => job::handle(o),
        Command::Account(o) => account::handle(o),
        Command::Search(o) => search::handle(o),
    }
}
