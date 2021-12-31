use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ToDo {
    content: String,
    finished: bool,
}
