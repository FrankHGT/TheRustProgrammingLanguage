pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

// when you have a lifetime of struct member, 
// you always need lifetime generic in impl
impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}