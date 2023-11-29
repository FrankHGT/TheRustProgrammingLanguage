mod type_alias;
mod never;
mod dynamically_sized_types;

pub fn run() {
    type_alias::run();
    never::run();
    dynamically_sized_types::run();
}