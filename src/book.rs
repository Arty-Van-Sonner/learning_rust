
#[derive(Debug)] // need for outpute through "{:?}"
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
    pub pages: u32,
}
impl Book {
    pub fn new(title: &str, author: &str, year: u32, pages: u32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
            pages,
        }
    }
}