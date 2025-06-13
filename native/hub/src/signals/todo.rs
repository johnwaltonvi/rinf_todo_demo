use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

/// Represents a single todo item
#[derive(Serialize, Deserialize, Clone, SignalPiece)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

/// Signal sent from Rust to Dart containing the list of todo items
#[derive(Serialize, RustSignal)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub pending_count: u32,
}

/// Signal sent from Dart to Rust to add a new todo item
#[derive(Deserialize, DartSignal)]
pub struct AddTodo {
    pub text: String,
}

/// Signal sent from Dart to Rust to toggle a todo item's completion status
#[derive(Deserialize, DartSignal)]
pub struct ToggleTodo {
    pub id: u32,
}

/// Signal sent from Dart to Rust to delete a todo item
#[derive(Deserialize, DartSignal)]
pub struct DeleteTodo {
    pub id: u32,
}

/// Signal sent from Dart to Rust to request the current todo list
#[derive(Deserialize, DartSignal)]
pub struct GetTodos;
