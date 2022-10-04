use std::io::Error;

pub trait Draw {
    fn draw(&self);
}

pub trait PerformOnSdbx {

    fn run(&self){
        let name = self.get_name();
        println!("-- {}: start", name);
        match self.perform() {
            Ok(()) => println!("-- {}: done", name),
            Err(e) => println!("-- {}: failed [{}]", name, e.to_string()),
        }
    }

    fn perform(&self) -> Result<(), Error>;

    fn get_name(&self) -> &str;
    
}