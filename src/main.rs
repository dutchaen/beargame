use std::thread;
use std::time::Duration;

use read_input::shortcut::input_inside;

fn main() {

    // Create an interactive fiction/text adventure game
    println!("welcome to a game");
    thread::sleep(Duration::from_secs(2));

    println!("you are stuck in a forest, for some reason...");
    thread::sleep(Duration::from_secs(1));

    println!("will you walk forward or will you stay where you are? [1 or 2]: ");
    let option = input_inside(1..3);

    if option == 2 {
        die("yeah you died (oops...)");
    }

    println!("you see a bear... what do you do? will you attack it or will you be its friend? [1 or 2]");
    let option = input_inside(1..3);

    if option == 1 {
        die("you shouldnt have done that, bears are stronger than humans (u died)");
    }

    println!("you see a city, will you leave your bear behind or will you take it with you? [1 or 2]");
    let option = input_inside(1..3);

    if option == 1 {
        die("that bear really loved you so i am going to kill u just for being selfish (u died)");
    }

    println!("you win :DDDD");
    thread::sleep(Duration::from_secs(7));

}


fn die(msg: &str) {
    println!("{}", msg);
    thread::sleep(Duration::from_secs(5));
    std::process::exit(-1);
}
