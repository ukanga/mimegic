extern crate tree_magic;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let result = tree_magic::from_filepath(args.path.as_path());
    println!("The file {} is of mimetype {}.", args.path.display(), result);
}
