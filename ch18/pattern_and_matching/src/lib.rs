mod iflet;
mod whilelet;
mod for_pattern;
mod let_pattern;
mod fn_parameter;
mod refutable;
mod matching;

pub fn run() {
    iflet::run();
    whilelet::run();
    for_pattern::run();
    let_pattern::run();
    fn_parameter::run();
    refutable::run();
    matching::run();
}