use crate::core::todo::Todo;
use rusqlite::Connection;

pub(crate) struct Storage {
    connection: Connection,
}

impl Storage {
    pub(crate) fn get_all_todos(&self) -> Vec<Todo> {
        let mut statement = self
            .connection
            .prepare("SELECT id, title, content, completed FROM todo")
            .unwrap();
        let todos = statement
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    content: row.get(2).unwrap(),
                    completed: row.get(3).unwrap(),
                })
            })
            .unwrap();
        todos.map(|todo| todo.unwrap()).collect()
    }
    
    pub(crate) fn insert_todo(&self, todo: &Todo) {
        self.connection
            .execute(
                "INSERT INTO todo (title, content, completed) VALUES (?1, ?2, ?3)",
                rusqlite::params![todo.title, todo.content, todo.completed],
            )
            .unwrap();
    }
    
    pub(crate) fn delete_all_todos(&self) {
        self.connection
            .execute("DELETE FROM todo", [])
            .unwrap();
    }
}

impl Storage {
    pub(crate) fn new(file_path: String) -> Self {
        let connection = Connection::open(file_path).unwrap();
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS todo (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            )",
                [],
            )
            .unwrap();
        Self { connection }
    }
}
