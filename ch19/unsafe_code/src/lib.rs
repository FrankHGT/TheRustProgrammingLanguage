mod raw_pointer;
mod unsafe_function;
mod static_variable;
mod unsafe_trait;

pub fn run() {
    raw_pointer::run();
    unsafe_function::run();
    static_variable::run();
    unsafe_trait::run();
}