mod code_container_test;

use code_container_test::TestCodeContainer;
use sdbx_shared::code_runner::CodeRunner;

pub fn run_code_containers() {
    CodeRunner::run::<TestCodeContainer>();
}
