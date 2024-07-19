use bitflags::bitflags;
use introduce_macro_derive::Introducer;
use sdbx_commons::PerformOnSdbx;
use sdbx_generics_concept::*;
use sdbx_macros::Introducer;
use std::cmp::Ordering;
use std::io::{Error, ErrorKind};

bitflags! {

    pub struct NmbFlags: u32 {

        const ONE = 0x01;
        const TWO = 0x02;
        const THREE = 0x04;
        const FOUR = 0x08;
    }

}

/* misc testing */
#[derive(Introducer)]
pub struct TestMisc;

impl PerformOnSdbx for TestMisc {
    fn perform(&self) -> Result<(), std::io::Error> {
        let mut owned_string: String = "hello ".to_owned();

        owned_string.push_str(" there");

        println!("got that from getStringByOption: {}", owned_string);

        Ok(())
    }
}

#[derive(Introducer)]
pub struct TestFlags;

impl PerformOnSdbx for TestFlags {
    fn perform(&self) -> Result<(), Error> {
        let mut my_flags = NmbFlags::empty();

        macro_rules! check_fag {
            ($insert:expr, $check:expr, $err:expr) => {
                my_flags.set($insert, true);
                if !my_flags.contains($check) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("flag failed for {}", $err),
                    ));
                }
            };
            ($check:expr, $err:expr) => {
                if !my_flags.contains($check) {
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("flag failed for {}", $err),
                    ));
                }
            };
            ($check:expr) => {
                if !my_flags.contains($check) {
                    return Err(Error::new(ErrorKind::Other, "flag failed for any reason"));
                }
            };
        }

        check_fag!(NmbFlags::ONE, NmbFlags::ONE, "NmbFlags::TWO");
        check_fag!(NmbFlags::TWO, NmbFlags::TWO, "NmbFlags::TWO");
        check_fag!(NmbFlags::TWO);

        Ok(())
    }
}

/* error handling tests */

#[derive(Introducer)]
pub struct TestErrorHandling;

impl PerformOnSdbx for TestErrorHandling {
    fn perform(&self) -> Result<(), Error> {
        let index = 41;

        if index > 42 {
            return Err(Error::new(ErrorKind::Other, "oh no!"));
        }

        Ok(())
    }
}

/* test pointer on string */

#[derive(Introducer)]
pub struct TestStringPointer;

impl PerformOnSdbx for TestStringPointer {
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

#[derive(Introducer)]
pub struct TestOrdering;

impl PerformOnSdbx for TestOrdering {
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

#[derive(Introducer)]
pub struct TestVector;

#[allow(clippy::needless_range_loop)]
#[allow(clippy::single_match)]
impl PerformOnSdbx for TestVector {
    fn perform(&self) -> Result<(), std::io::Error> {
        let vec = vec!["one", "two", "three"];

        for str in &vec {
            println!("{}", str);
        }

        for i in 0..vec.len() {
            println!("{} in forinrange:{}", i + 1, vec[i]);
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

/* test generics concept */

#[derive(Introducer)]
pub struct TestGenericsConceptTry;

impl PerformOnSdbx for TestGenericsConceptTry {
    fn perform(&self) -> Result<(), std::io::Error> {
        run_generics_try();

        Ok(())
    }
}

/* test generics in collection */

#[derive(Introducer)]
pub struct TestGenericsConceptCollection;

impl PerformOnSdbx for TestGenericsConceptCollection {
    fn perform(&self) -> Result<(), std::io::Error> {
        run_generics_collection();

        Ok(())
    }
}
