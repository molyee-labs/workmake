use crate::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    scope: Vec<String>,
    exclude: Vec<String>,
    user: Option<String>,
    pattern: String,
}

pub(crate) fn handle(opts: Args) -> Result<()> {

}
