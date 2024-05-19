


// Create a function create random number
pub fn create_random_number() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(1..101)
}

// Create a function to get number from user
pub fn get_number_from_user() -> u32 {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    guess
}


// Create a function to check the guess
pub fn check_guess(guess: u32, random_number: u32) -> bool {
    if guess == random_number {
        true
    } else {
        false
    }
}
