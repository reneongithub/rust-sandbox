use sdbx_shared::code_runner::CodeContainer;

#[derive(Default)]
pub struct TestCodeContainer;

impl CodeContainer for TestCodeContainer {
    fn run(&self) {
        println!("test code container runs by shared code runner")
    }
}
