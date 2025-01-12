mod cars;
mod sneko;
mod systems;

use std::env;

// example command `cargo run sneko`
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please specify a game name");
        return;
    }

    if args.len() > 2 {
        println!("Too many args, just specify a game name");
        return;
    }

    let game_name = &args[1];
    println!("Lounching game {}", game_name);
    match game_name.as_str() {
        "sneko" => sneko::lounch_snake(),
        "cars" => cars::launch_cars(),
        _ => {
            println!("This game does not exist");
        }
    }
    if game_name == "sneko" {
        sneko::lounch_snake();
    } else {
    }
    //
    // cars::launch_cars();
}
