fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0 as i32;
    } else if guess > secret {
        return 1 as i32;
    } else {
        return -1 as i32;
    }
}

fn main() {
    let secret = 42;
    let mut user_guess: i32 = 52;
    let mut user_tries: i32 = 0;
    while user_guess != secret {
        if check_guess(user_guess, secret) == 0 {
            break;
        } else if check_guess(user_guess, secret) > 0 {
            user_guess -= 1;
            user_tries += 1;
        } else if check_guess(user_guess, secret) < 0 {
            user_guess += 1;
            user_tries += 1;
        }
    }
    println!("It took {} attempts to guess the right number!", user_tries);
}
