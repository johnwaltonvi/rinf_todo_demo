use crate::signals::{TodoCommand, TodoItem, TodoList};
use crate::AppState;
use rinf::debug_print;
use rinf_router::{State, enable_direct_return};

// Enable direct return for TodoList
enable_direct_return!(TodoList);

// Helper function to create the current todo list
async fn create_todo_list(app_state: &AppState) -> TodoList {
    let todos = app_state.todos.lock().await;
    let pending_count = todos.iter().filter(|item| !item.completed).count() as u32;
    TodoList {
        items: todos.clone(),
        pending_count,
    }
}

/// Unified handler for all TodoCommand signals
pub async fn handle_todo_command(
    State(app_state): State<AppState>,
    cmd: TodoCommand,
) -> TodoList {  // 👈 Clean return type!
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
        }
        TodoCommand::Delete { id } => {
            debug_print!("Router received TodoCommand::Delete for id: {}", id);

            // Remove todo
            {
                let mut todos = app_state.todos.lock().await;
                todos.retain(|t| t.id != id);
            }
        }
        TodoCommand::GetAll => {
            debug_print!("Router received TodoCommand::GetAll");
            // No state changes needed for GetAll
        }
    }

    // Clean return - automatically sent to Dart!
    create_todo_list(&app_state).await
}