pub struct Crud{}

impl Crud{

    pub fn insert(&self, value: &String) -> bool {
        println!("insert: {}", value);

        value.len() > 1
    }

    pub fn update(&self, value: &String) -> bool {
        println!("update: {}", value);

        value.len() > 1
    }

    pub fn delete(&self, value: &String) -> bool {
        println!("delete: {}", value);

        value.len() > 1
    }

    pub fn select(&self, value:  &String) -> Option<String> {
    
        let mut ret = String::from("selected String: ");
        ret.push_str(value);
    
        Option::Some(String::from(ret))
    }

}