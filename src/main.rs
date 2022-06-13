use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    play_around();
}

fn print_to_console(int_to_console: i32, string_to_console: &String){

    println!("from console - int_a: {} -- string_a: {}", int_to_console, string_to_console);
}

fn play_around(){

    println!("ENTER play_around");

    print_to_console(2, &String::from("to console"));

    let mut owned_string: String = "hello ".to_owned();
    
    owned_string.push_str(" there");

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

    let opt_string = get_string_by_option(&owned_string).unwrap();

    println!("got that from getStringByOption: {}", opt_string);

    println!("EO play_around");

    let mut str = String::from("string to trans;");

    let str_pt = &mut str;

    str_pt.push_str("string;");

    extend_string(str_pt);

    println!("{}",str);

}

fn get_string_by_option(conv_string: &String) -> Option<String> {
    
    let mut ret = String::from("return String: ");
    ret.push_str(conv_string);

    Option::Some(String::from(ret))
}

fn extend_string(conv_string: &mut String){
    conv_string.push_str("string_extended;");
}
