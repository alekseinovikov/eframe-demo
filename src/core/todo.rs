#[derive(Debug, Clone)]
pub struct Todo {
    pub(crate) id: Option<i64>,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) completed: bool,
}

impl Todo {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: None,
            title,
            content,
            completed: false,
        }
    }
}
