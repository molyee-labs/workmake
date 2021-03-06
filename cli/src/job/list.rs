use clap::Parser;

#[derive(Parser)]
pub struct Args {
    user: Option<user::Id>,
    company: Option<company::Id>,
    project: Option<project::Id>,
}