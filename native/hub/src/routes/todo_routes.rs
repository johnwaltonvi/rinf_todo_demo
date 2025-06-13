use crate::signals::{TodoCommand, TodoItem, TodoList};
use crate::AppState;
use rinf::{debug_print, RustSignal};
use rinf_router::State;

// Helper function to send the current todo list to Dart
async fn send_todo_list(app_state: &AppState) {
    let todos = app_state.todos.lock().await;
    let pending_count = todos.iter().filter(|item| !item.completed).count() as u32;
    TodoList {
        items: todos.clone(),
        pending_count,
    }.send_signal_to_dart();
}

/// Unified handler for all TodoCommand signals
pub async fn handle_todo_command(
    State(app_state): State<AppState>,
    cmd: TodoCommand,
) {
    match cmd {
        TodoCommand::Add { text } => {
            debug_print!("Router received TodoCommand::Add: {}", text);

            // Create new todo item
            let mut next_id = app_state.next_id.lock().await;
            let new_todo = TodoItem {
                id: *next_id,
                text,
                completed: false,
            };
            *next_id += 1;

            // Add to todos
            {
                let mut todos = app_state.todos.lock().await;
                todos.push(new_todo);
            }

            // Send updated list to Dart
            send_todo_list(&app_state).await;
        }
        TodoCommand::Toggle { id } => {
            debug_print!("Router received TodoCommand::Toggle for id: {}", id);

            // Toggle todo completion status
            {
                let mut todos = app_state.todos.lock().await;
                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    todo.completed = !todo.completed;
                }
            }

            // Send updated list to Dart
            send_todo_list(&app_state).await;
        }
        TodoCommand::Delete { id } => {
            debug_print!("Router received TodoCommand::Delete for id: {}", id);

            // Remove todo
            {
                let mut todos = app_state.todos.lock().await;
                todos.retain(|t| t.id != id);
            }

            // Send updated list to Dart
            send_todo_list(&app_state).await;
        }
        TodoCommand::GetAll => {
            debug_print!("Router received TodoCommand::GetAll");

            // Send current list to Dart
            send_todo_list(&app_state).await;
        }
    }
}
