mod associated_type;
mod fully_qualified_syntax;
mod supertrait;
mod newtype;

pub fn run() {
    associated_type::run();
    fully_qualified_syntax::run();
    supertrait::run();
    newtype::run();
}