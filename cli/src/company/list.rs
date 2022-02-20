use clap::Parser;

#[derive(Parser)]
pub struct Args {
    company_id: String,
    name: String,
}