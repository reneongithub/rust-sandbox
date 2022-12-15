pub fn main_run() {
    dosome();
}

trait Printable {
    fn printme(&mut self);
}

enum Ani<E> {
    One(E),
    Two,
}

impl Ani<String> {
    fn printout(self) {
        match self {
            Self::One(s) => println!("{}", s),
            Self::Two => println!("two"),
        }
    }
}

impl<T: Printable> Ani<T> {
    fn printout(self) {
        match self {
            Self::One(mut s) => s.printme(),
            Self::Two => println!("two"),
        }
    }
}

pub struct PrinterA {
    str_field: String,
}

impl PrinterA {
    pub fn new(str_field: String) -> Self {
        PrinterA {
            str_field: (str_field),
        }
    }
}

impl Printable for PrinterA {
    fn printme(&mut self) {
        println!("{}", self.str_field)
    }
}

fn dosome() {
    let ani_string: Ani<String> = Ani::One(String::from("new string"));

    ani_string.printout();

    let ani_blank: Ani<String> = Ani::Two;

    ani_blank.printout();

    let mut printer_a = PrinterA::new(String::from("string on init"));

    printer_a.printme();

    let ani_print: Ani<PrinterA> = Ani::One(PrinterA::new(String::from("string on init")));
    ani_print.printout();

    // let one = Ani<E,F>::One("in one");
    // one.printout();

    // let two = Ani::Two("in two");
    // two.printout();
}
