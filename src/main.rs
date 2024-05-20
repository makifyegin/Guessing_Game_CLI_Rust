use clap::Parser;

//use unit_testing::guessing_game;

#[derive(Parser)]
#[clap(version = "1.0", author = "Akif", about = "Guessing game")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(name = "play", about = "Play the game")]
    Play {
        #[clap(short, long)]
        number: Option<u32>,
    },
    #[clap(name = "generate", about = "Generate a random number")]
    Generate,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { number }) => {
            let random_number = guessing_game::create_random_number();
            let guess = number.unwrap_or_else(guessing_game::get_number_from_user);

            println!("Your guess is: {}", guess);
            let result = guessing_game::check_guess(guess, random_number);
            if result {
                println!("You win!");
            } else {
                println!("You lose!");
            }
        }
        Some(Commands::Generate) => {
            let random_number = guessing_game::create_random_number();
            println!("Random number is: {}", random_number);
        }
        None => {
            println!("Please provide a command");
        }
    }
}
