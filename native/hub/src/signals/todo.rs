use rinf::{DartSignal, RustSignal, SignalPiece};
use serde::{Deserialize, Serialize};

/// Unified command enum for all todo operations sent from Dart to Rust
#[derive(Deserialize, DartSignal)]
pub enum TodoCommand {
    Add { text: String },
    Toggle { id: u32 },
    Delete { id: u32 },
    GetAll,
}

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
