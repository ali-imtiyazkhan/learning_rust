enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn main() {
    // --- The match Control Flow Operator ---
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value: {}", value_in_cents(coin));

    // --- Matching with Option<T> ---
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Six: {:?}, None: {:?}", six, none);

    // --- The _ Placeholder ---
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // catch-all
    }

    // --- if let Syntax ---
    // Use if let when you only care about one pattern and want to ignore the rest.
    let config_max = Some(3u8);
    
    // Instead of:
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // Use:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No max configured");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
