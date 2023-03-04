mod player;
use introduce_macro_derive::Introducer;
use sdbx_macros::Introducer;

#[derive(Introducer)]
pub struct ImNew;

fn main() {
    player::Player::play();
    play();
}

fn play() {
    let the_new = ImNew;
    the_new.introduce();

    let mut s: String = String::from("doo");
    s.push_str("-extended");

    let clo = || {
        // s.push_str("-extended");
        println!("within closures................{}", s);
    };

    println!("{}", s);

    clo();
}
