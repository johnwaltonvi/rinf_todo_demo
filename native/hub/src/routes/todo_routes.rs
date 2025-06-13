use crate::signals::{AddTodo, DeleteTodo, GetTodos, ToggleTodo, TodoItem, TodoList};
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

/// Handler for AddTodo signal
pub async fn handle_add_todo(
    State(app_state): State<AppState>,
    msg: AddTodo,
) {
    debug_print!("Router received AddTodo: {}", msg.text);

    // Create new todo item
    let mut next_id = app_state.next_id.lock().await;
    let new_todo = TodoItem {
        id: *next_id,
        text: msg.text,
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

/// Handler for ToggleTodo signal
pub async fn handle_toggle_todo(
    State(app_state): State<AppState>,
    msg: ToggleTodo,
) {
    debug_print!("Router received ToggleTodo for id: {}", msg.id);

    // Toggle todo completion status
    {
        let mut todos = app_state.todos.lock().await;
        if let Some(todo) = todos.iter_mut().find(|t| t.id == msg.id) {
            todo.completed = !todo.completed;
        }
    }

    // Send updated list to Dart
    send_todo_list(&app_state).await;
}

/// Handler for DeleteTodo signal
pub async fn handle_delete_todo(
    State(app_state): State<AppState>,
    msg: DeleteTodo,
) {
    debug_print!("Router received DeleteTodo for id: {}", msg.id);

    // Remove todo
    {
        let mut todos = app_state.todos.lock().await;
        todos.retain(|t| t.id != msg.id);
    }

    // Send updated list to Dart
    send_todo_list(&app_state).await;
}

/// Handler for GetTodos signal
pub async fn handle_get_todos(
    State(app_state): State<AppState>,
    msg: GetTodos,
) {
    debug_print!("Router received GetTodos");

    // Send current list to Dart
    send_todo_list(&app_state).await;
}
