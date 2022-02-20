use clap::Parse;

#[derive(Parse)]
pub struct SearchOpts {
    scope: Vec<String>,
    exclude: Vec<String>,
    pattern: String,
}
