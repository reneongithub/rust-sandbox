pub trait Introducer {
    fn introduce(&self);
    fn get_type_name(&self) -> String;
}
