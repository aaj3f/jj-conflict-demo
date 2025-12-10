use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A simple greeting application")]
struct Args {
    // No arguments yet
}

fn main() {
    let _args = Args::parse();
    println!("Hello, world!");
}
