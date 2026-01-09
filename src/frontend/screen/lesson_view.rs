#[derive(Debug)]
pub enum Message {}

#[derive(Debug, Clone)]
pub struct State {
    pub title: String,
    pub path: String,
    pub slide_count: u8,
}

impl State {
    pub fn new(title: String, path: String, slide_count: u8) -> Self {
        Self {
            title,
            path,
            slide_count,
        }
    }
}
