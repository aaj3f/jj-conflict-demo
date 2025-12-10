use clap::Parser;

#[derive(Parser)]
#[command(name = "greeting")]
#[command(about = "A simple greeting application")]
struct Args {
    /// Name of the user to greet
    #[arg(short, long, default_value = "World")]
    user: String,

    #[arg(long)]
    json: bool,
}

struct Output {
    message: String,
}

impl Output {
    fn to_json(&self) -> String {
        format!("{{\"message\": \"{}\"}}", self.message)
    }

    fn to_plain_text(&self) -> String {
        self.message.clone()
    }
}

fn main() {
    let args = Args::parse();
    let json_output = args.json;

    let output = Output {
        message: format!("Hello, {}!", args.user),
    };

    if json_output {
        println!("{}", output.to_json());
    } else {
        println!("{}", output.to_plain_text());
    }
}
