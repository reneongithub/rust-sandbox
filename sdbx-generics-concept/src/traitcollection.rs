use std::io::{Error, ErrorKind};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub fn main_run() {
    dosome();
}

/* Traits */

trait PrintFields {
    fn print_all_field(&self) -> Result<(), Error>;
}

/* Useless */

struct UselessStruct;

impl PrintFields for UselessStruct {
    fn print_all_field(&self) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, "no fields present"))
    }
}

/* OneField */

struct OneFieldStruct {
    v_a: i32,
}

impl OneFieldStruct {
    pub fn new(val: i32) -> Self {
        OneFieldStruct { v_a: val }
    }
}

impl PrintFields for OneFieldStruct {
    fn print_all_field(&self) -> Result<(), Error> {
        println!("OneFieldStruct fields: {}", self.v_a);

        Ok(())
    }
}

/* TowField */

struct TwoFieldStruct {
    v_a: i32,
    v_b: i32,
}

impl TwoFieldStruct {
    pub fn new(val_a: i32, val_b: i32) -> Self {
        TwoFieldStruct {
            v_a: val_a,
            v_b: val_b,
        }
    }
}

impl PrintFields for TwoFieldStruct {
    fn print_all_field(&self) -> Result<(), Error> {
        println!("TwoFieldStruct fields: {} : {}", self.v_a, self.v_b);

        Ok(())
    }
}

impl<X> PrintFields for X
where
    X: Deref,
    X::Target: PrintFields,
{
    fn print_all_field(&self) -> Result<(), Error> {
        self.deref().print_all_field()
    }
}

fn dosome() {
    let useless = UselessStruct;
    match useless.print_all_field() {
        Ok(()) => println!("done"),
        Err(e) => println!("failed [{}]", e),
    }

    let onefield = OneFieldStruct::new(24);
    match onefield.print_all_field() {
        Ok(()) => println!("done"),
        Err(e) => println!("failed [{}]", e),
    }

    let twofield = TwoFieldStruct::new(42, 24);
    match twofield.print_all_field() {
        Ok(()) => println!("done"),
        Err(e) => println!("failed [{}]", e),
    }

    let coll: Vec<Rc<dyn PrintFields>> = vec![
        Rc::new(UselessStruct),
        Rc::new(OneFieldStruct::new(24)),
        Rc::new(TwoFieldStruct::new(42, 24)),
    ];

    delegate_print_fields_iter(&coll);

    let coll: Vec<Arc<dyn PrintFields>> = vec![
        Arc::new(UselessStruct),
        Arc::new(OneFieldStruct::new(24)),
        Arc::new(TwoFieldStruct::new(42, 24)),
    ];

    delegate_print_fields_iter(&coll);

    let coll: Vec<Box<dyn PrintFields>> = vec![
        Box::new(UselessStruct),
        Box::new(OneFieldStruct::new(24)),
        Box::new(TwoFieldStruct::new(42, 24)),
    ];

    delegate_print_fields_iter(&coll);

    let coll: Vec<&dyn PrintFields> = vec![&useless, &onefield, &twofield];

    delegate_print_fields_iter(&coll);

    let coll: &[&dyn PrintFields] = &mut [&useless, &onefield, &twofield];

    delegate_print_fields_iter(coll);
}

fn delegate_print_fields_iter<I>(coll: I)
where
    I: IntoIterator,
    I::Item: Deref,
    <I::Item as Deref>::Target: PrintFields,
{
    for item in coll {
        delegate_print_fields(item.deref())
    }
}

fn delegate_print_fields<T>(item: T)
where
    T: PrintFields,
{
    match item.print_all_field() {
        Ok(()) => println!("done"),
        Err(e) => println!("failed [{}]", e),
    }
}
