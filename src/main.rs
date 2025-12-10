use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A simple greeting application")]
struct Args {
    /// Name of the user to greet
    #[arg(short, long, default_value = "World")]
    user: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.user);
}
