fn main() {
    println!("Hello, world!");
    // Create a random number
    let random_number = guessing_game::create_random_number();
    println!("Random number is: {}", random_number);

    // Get number from user
    let guess = guessing_game::get_number_from_user();
    println!("Your guess is: {}", guess);

    // Check the guess
    let result = guessing_game::check_guess(guess, random_number);
    if result {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
