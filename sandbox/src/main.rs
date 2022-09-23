mod business;
mod player;

use business::sub;
//use business::Crud;


fn main() {
    println!("Hello, world!");

    player::play_around();


    sub::helper_one_me("me");

    use test_module_concept::ModOneService as serv_one;
    serv_one::test_mod_one_service();

    //let cr1 = business::crud::Crud{};

}
