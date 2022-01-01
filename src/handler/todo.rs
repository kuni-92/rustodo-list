use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ToDo {
    content: String,
    finished: bool,
}

impl ToDo {
    pub fn new(content: String) -> Self {
        ToDo{content, finished: false}
    }
}
