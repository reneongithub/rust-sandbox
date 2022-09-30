mod business;
mod player;

use business::sub;
//use business::Crud;


fn main() {
    println!("Hello, world!");

    player::play_around();


    sub::helper_one_me("me");

    use sdbx_module_concept::ModOneService as serv_one;
    serv_one::test_mod_one_service();

    //let cr1 = business::crud::Crud{};

    test_gui();

}


fn test_gui(){

    use sdbx_commons::{Button,Screen,Draw};

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    
    impl Draw for SelectBox {
        fn draw(&self) {
            println!("drew select box to screen with width of {}", self.width);
        }
    }

    let my_button = Button{
        width: 10,
        height: 20,
        label: String::from("i am a button"),
    };

    let my_select = SelectBox {
        width: 30,
        height: 40,
        options: vec![
            String::from("option one"),
            String::from("option two")
        ],
    };


    // let screen = Screen{

    //     components: vec![
    //         Box::new(my_button),
    //         Box::new(my_select),
    //     ]
    // };

    // screen.run();

    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(my_button),
        Box::new(my_select),
    ];

    // following will not compile ...
    // -- use of moved value: `my_select` value used here after moverustcE0382 --
    // let components2: Vec<Box<dyn Draw>> = vec![
    //     Box::new(my_button),
    //     Box::new(my_select),
    // ];


    for component in components.iter() {
        component.draw();
    };


    // let mut components: Vec<Box<dyn Draw>> = Vec::new();

    // components.push(Box::new(
    //     Button{
    //         width: 10,
    //         height: 20,
    //         label: String::from("i am a button"),
    //     }
    // ));

    // components.push(Box::new(
    //     SelectBox{
    //         width: 30,
    //         height: 40,
    //         options: vec![
    //             String::from("option one"),
    //             String::from("option two")
    //         ],
    //     }
    // ));





}
