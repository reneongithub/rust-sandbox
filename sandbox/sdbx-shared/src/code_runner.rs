pub trait CodeContainer {
    fn run(&self);
}

pub struct CodeRunner;

impl CodeRunner {
    pub fn run<T>()
    where
        T: CodeContainer + Default,
    {
        let instance = T::default();
        instance.run();
    }
}
