use sdbx_commons::PerformOnSdbx;
use std::cmp::Ordering;
use std::io::{Error, ErrorKind};

/* misc testing */

pub struct TestMisc {
    pub name: String,
}

impl PerformOnSdbx for TestMisc {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn perform(&self) -> Result<(), std::io::Error> {
        let mut owned_string: String = "hello ".to_owned();

        owned_string.push_str(" there");

        println!("got that from getStringByOption: {}", owned_string);

        Ok(())
    }
}

/* error handling tests */

pub struct TestErrorHandling {
    pub name: String,
}

impl PerformOnSdbx for TestErrorHandling {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn perform(&self) -> Result<(), Error> {
        let index = 41;
        println!("{}", self.name);

        if index > 42 {
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }

        Ok(())
    }
}

/* test pointer on string */

pub struct TestStringPointer {
    pub name: String,
}

impl PerformOnSdbx for TestStringPointer {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn perform(&self) -> Result<(), std::io::Error> {
        let mut str = String::from("string to trans;");

        Self::print_to_console(2, &String::from("to console"));

        let str_pt = &mut str;

        str_pt.push_str("string;");

        Self::extend_string(str_pt);

        println!("{}", str);

        let mut owned_string: String = "hello ".to_owned();
        owned_string.push_str(" there");
        let opt_string = Self::get_string_by_option(&owned_string).unwrap();

        println!("got that from getStringByOption: {}", opt_string);

        Ok(())
    }
}

impl TestStringPointer {
    fn print_to_console(int_to_console: i32, string_to_console: &String) {
        println!(
            "from console - int_a: {} -- string_a: {}",
            int_to_console, string_to_console
        );
    }

    fn extend_string(conv_string: &mut String) {
        conv_string.push_str("string_extended;");
    }

    fn get_string_by_option(conv_string: &str) -> Option<String> {
        let mut ret = String::from("return String: ");
        ret.push_str(conv_string);

        Option::Some(ret)
    }
}

/* test ordering */

pub struct TestOrdering {
    pub name: String,
}

impl PerformOnSdbx for TestOrdering {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn perform(&self) -> Result<(), std::io::Error> {
        let int_a = 1;

        let cmp_result = 1.cmp(&int_a);

        match cmp_result {
            Ordering::Equal => println!("Equal from match"),
            Ordering::Greater => println!("Equal from match"),
            Ordering::Less => println!("Equal from match"),
        }

        if cmp_result.is_eq() {
            println!("Equal from if condition")
        }

        if cmp_result == Ordering::Equal {
            println!("Equal from if condition == ")
        }

        Ok(())
    }
}

/* test vector */

pub struct TestVector {
    pub name: String,
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::single_match)]
impl PerformOnSdbx for TestVector {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn perform(&self) -> Result<(), std::io::Error> {
        let vec = vec!["one", "two", "three"];

        for str in &vec {
            println!("{}", str);
        }
        
        for i in 0..vec.len() {
            println!("{} in forinrange:{}", i+1, vec[i]);
        }

        for i in 0..vec.len() + 1 {
            match vec.get(i) {
                Some(v) => println!("forinmatch:{}", v),
                None => println!(
                    "forinmatch oob on index:{} --- caught and continue.. ;-)",
                    i
                ),
            }
        }

        let val1 = vec.get(1);
        match val1 {
            Some(v) => println!("{}", v),
            _ => (),
        }

        if let Some(v) = val1 {
            println!("{}", v);
        }

        Ok(())
    }
}

// fn print_to_console(int_to_console: i32, string_to_console: &String) {
//     println!(
//         "from console - int_a: {} -- string_a: {}",
//         int_to_console, string_to_console
//     );
// }

// fn extend_string(conv_string: &mut String) {
//     conv_string.push_str("string_extended;");
// }

// fn get_string_by_option(conv_string: &str) -> Option<String> {
//     let mut ret = String::from("return String: ");
//     ret.push_str(conv_string);

//     Option::Some(ret)
// }
