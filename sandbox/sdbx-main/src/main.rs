mod player;

use introduce_macro_derive::Introducer;
use sdbx_shared::code_runner::{ CodeContainer, CodeRunner };
use sdbx_macros::Introducer;

#[derive(Introducer)]
pub struct ImNew;

#[derive(Default)]
struct TestCodeContainer;

impl CodeContainer for TestCodeContainer {
    
    fn run(&self){
        println!("test code container runs by shared code runner")
    }
}

fn main() {

    CodeRunner::run::<TestCodeContainer>();

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
