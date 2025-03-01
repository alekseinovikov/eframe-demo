use crate::core::todo::Todo;
use crate::storage::Storage;

pub(crate) struct Service {
    storage: Storage,
}

impl Service {
    pub(crate) fn get_all_todos(&self) -> Vec<Todo> {
        self.storage.get_all_todos()
    }
}

impl Service {
    pub(crate) fn new(storage: Storage) -> Self {
        // todo: remove once we implemented creation of todos
        storage.delete_all_todos();
        for i in 0..10 {
            storage.insert_todo(&Todo::new(
                format!("Todo {}", i),
                "Lorem ipsum dolor sit amet".to_string(),
            ));
        }

        Self { storage }
    }
}
