

    pub use helper as helper_exp;
    pub use helper::help_me as helper_help_me_exp;

    pub mod helper{
        pub fn help_me(name: &str){
            println!("{} - you will get help", name);
        }
    }

    pub fn do_some_business() {
        
        println!("doing some business")
    }