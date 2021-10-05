// https://rust-cli.github.io/book/

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).unwrap();

    // TODO optimize this https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}
