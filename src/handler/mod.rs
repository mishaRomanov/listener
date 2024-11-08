#[derive(Debug)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
    pub fn greet(&self) -> &'static str {
        "Hello, I am handler!"
    }
}
