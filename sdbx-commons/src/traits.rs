use std::io::Error;

pub trait PerformOnSdbx {
    fn run(&self) {
        let name = self.get_name();
        println!("\n-- {}: start", name);
        match self.perform() {
            Ok(()) => println!("-- {}: done", name),
            Err(e) => println!("-- {}: failed [{}]", name, e),
        }
    }

    fn perform(&self) -> Result<(), Error>;
    fn get_name(&self) -> &str;
}
