
use std::{cmp::Ordering, slice::SliceIndex};
use crate::business;

pub fn play_around(){

    println!("ENTER play_around\n");

    test_misc();

    test_pointer_on_string();

    test_ordering();

    test_use_keyword();

    test_vector();


    println!("EO play_around");
}

fn test_misc(){

    println!("test misc__________________________");

    print_to_console(2, &String::from("to console"));

    let mut owned_string: String = "hello ".to_owned();
    
    owned_string.push_str(" there");




    let opt_string = get_string_by_option(&owned_string).unwrap();

    println!("got that from getStringByOption: {}", opt_string);

    println!("EOT misc__________________________");

}

fn test_pointer_on_string(){
    println!("test pointer_on_string__________________________");


    let mut str = String::from("string to trans;");

    let str_pt = &mut str;

    str_pt.push_str("string;");

    extend_string(str_pt);

    println!("{}",str);
    println!("EOT pointer_on_string__________________________");

}


fn test_ordering() {
    println!("test ordering__________________________");

    let int_a = 1;

    let cmp_result = 1.cmp(&int_a);

    match cmp_result {
        Ordering::Equal => println!("Equal from match"),
        Ordering::Greater => println!("Equal from match"),
        Ordering::Less => println!("Equal from match")
    }

    if cmp_result.is_eq() {
        println!("Equal from if condition")
    }

    if cmp_result == Ordering::Equal {
        println!("Equal from if condition == ")
    }

    println!("EOT ordering__________________________");
}

fn test_use_keyword() {
    println!("test use_keyword__________________________");

    business::do_some_business();
    business::helper::help_me("Rina");
    business::helper_exp::help_me("Dane");
    business::helper_help_me_exp("Ralf");

    println!("EOT use_keyword__________________________");
}

fn test_vector(){

    println!("test vector__________________________");

    let mut vec = Vec::new();

    vec.push("one");
    vec.push("tow");
    vec.push("three");

    for str in &vec {
        println!("{}", str);
    }

    for i in 0..vec.len() {
        println!("forinrange:{}",vec[i]);
    }

    for i in 0..vec.len()+1 {
        match vec.get(i) {
            Some(v) => println!("forinmatch:{}",v),
            None => println!("forinmatch oob on index:{} --- caught and continue.. ;-)",i)
        }
    }

    let val1 = vec.get(1);
    match val1 {

        Some(v) => println!("{}",v),
        _ => ()
        
    }

    if let Some(v) = val1 {
        println!("{}",v);
    }

    println!("EOT vector__________________________")

}


fn print_to_console(int_to_console: i32, string_to_console: &String){

    println!("from console - int_a: {} -- string_a: {}", int_to_console, string_to_console);
}

fn extend_string(conv_string: &mut String){
    conv_string.push_str("string_extended;");
}

fn get_string_by_option(conv_string: &String) -> Option<String> {
    
    let mut ret = String::from("return String: ");
    ret.push_str(conv_string);

    Option::Some(String::from(ret))
}