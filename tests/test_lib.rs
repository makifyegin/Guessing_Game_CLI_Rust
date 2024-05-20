// Create a unit test for the create_random_number function
use trust::guessing_game;
#[cfg(test)]




    #[test]
    fn test_create_random_number() {
        let random_number = guessing_game::create_random_number();
        assert!(random_number >= 1 && random_number <= 100);
    }

// Create a unit test for the get_number_from_user function



    #[test]
    fn test_get_number_from_user() {
        let guess = guessing_game::get_number_from_user();
        assert!(guess >= 1 && guess <= 100);
    }


