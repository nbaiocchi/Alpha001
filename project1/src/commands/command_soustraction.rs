use clap::Parser;

/// Options for the `outdated` command
#[derive(Parser, Debug)]
pub struct SoustractionOptions {
    #[clap(short = 'c', long)]
    /// Search only the Helm resources in terraform files
    pub chart: bool,
}

/// Print a tab that contain the source, actual version, latest version and path location
///
pub fn soustraction(_param: SoustractionOptions) {
    println!("This is the soustraction command !");
}