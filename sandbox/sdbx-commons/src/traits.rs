use ::sdbx_macros::Introducer;
use std::io::Error;

pub trait PerformOnSdbx: Introducer {
    fn run(&self) {
        let name = self.get_type_name();
        println!("\n-- {}: start", name);
        match self.perform() {
            Ok(()) => println!("-- {}: done", name),
            Err(e) => println!("-- {}: failed [{}]", name, e),
        }
    }

    fn perform(&self) -> Result<(), Error>;
}
