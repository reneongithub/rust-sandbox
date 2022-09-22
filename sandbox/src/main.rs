mod business;
mod player;

use business::sub;
//use business::Crud;

fn main() {
    println!("Hello, world!");

    player::play_around();


    sub::helper_one_me("me");

    //let cr1 = business::crud::Crud{};
}
