// Option is an Enum like Player or Bool
pub fn testing_option() {
    println!("Hello, world!");
    let x = divide(64.0, 8.34);
    dbg!(x);
    match x {
        // Option::Some(i) => println!("Result: {i}"),
        Some(i) => println!("Result: {i}"),
        Option::None => println!("Cannot divide by 0"),
    }

    let player = Player::Alive { life: 54 };
    let y = take_five(player);
    match y {
        Player::Alive { life } => println!("Alive with {life} life"),
        Player::KnockedOut {
            life,
            turns_to_wait,
        } => println!("KnockedOut at life of {life}, retries: {turns_to_wait}"),
        Player::Dead => println!("Dead"),
    }
}


pub fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        // 1. core::option::Option::None  --> very verbose
        // 2. Option::None  -> verbose
        None // standard way
    } else {
        let i = numerator / denominator;
        // 1. core::option::Option::Some(i)  -> very verbose
        // Option::Some(i)  -> verbose
        Some(i) // standard way
    }
}

enum PersonHealth {
    Dead,
    Alive,
}
struct Alive {}
enum Bool {
    True,
    False,
}

fn print_and_return_value(value: Bool) -> Bool {
    match value {
        Bool::True => {
            println!("Clearly true!");
            Bool::True
        }
        Bool::False => {
            println!("Unfortunately false!");
            Bool::False
        }
    }
}

enum Player {
    Alive { life: i8 },
    KnockedOut { life: i8, turns_to_wait: i8 },
    Dead,
}

fn take_five(player: Player) -> Player {
    match player {
        Player::Alive { life: renamed_life } => {
            if renamed_life - 5 >= 50 {
                Player::Alive {
                    life: renamed_life - 5,
                }
            } else {
                Player::KnockedOut {
                    life: renamed_life - 5,
                    turns_to_wait: 3,
                }
            }
        }
        Player::KnockedOut {
            life,
            turns_to_wait,
        } => Player::KnockedOut {
            life: life - 5,
            turns_to_wait,
        },
        Player::Dead => Player::Dead,
    }
}
