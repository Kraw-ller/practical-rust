use clap::Parser;
use colored::Colorize;

#[derive(Parser)]

struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message : String,
    #[clap(short = 'd', long = "dead", required = false)]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() {

    let options = Options::parse();
    let message = options.message;
    // let message = std::env::args().nth(1).expect("Missing the message. Usage: catsay <message>");
    let eye = if options.dead {"x"} else {"o"};
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .expect(&format!("could not read file {:?}", path));
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!(
                "{}",
                message.bright_yellow().underline().on_purple()
                );
            println!("{}", &cat_picture);
            },
            None => {
            // ... print the cat as before
            println!("{}", message.blue().on_purple());
            println!(" \\");
            println!("  \\");
            println!("  /\\_/\\");
            println!(" ( {eye} {eye} )", eye=eye.red().bold());
            println!(" =( I )=")
        }
    }
}
