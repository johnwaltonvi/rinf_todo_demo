//! This `hub` crate is the
//! entry point of the Rust logic.

mod actors;
mod routes;
mod signals;

use actors::create_actors;
use rinf::{dart_shutdown, debug_print, write_interface};
use rinf_router::{Router, State};
use crate::signals::{SmallText, BigBool};
use std::sync::{Arc, atomic::AtomicUsize};

#[cfg(not(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
)))]
pub use tokio;

#[cfg(all(
    target_arch = "wasm32",
    target_vendor = "unknown",
    target_os = "unknown"
))]
pub use tokio_with_wasm::alias as tokio;

use tokio::spawn;

write_interface!();

#[derive(Clone)]
pub struct AppState {
    pub first_actor: messages::prelude::Address<actors::FirstActor>,
    pub counter: Arc<AtomicUsize>,
    pub todos: Arc<tokio::sync::Mutex<Vec<signals::TodoItem>>>,
    pub next_id: Arc<tokio::sync::Mutex<u32>>,
}

// Router handlers that access app state
async fn handle_small_text(
    State(mut app_state): State<AppState>, 
    msg: SmallText
) {
    debug_print!("Router received SmallText: {}", msg.text);
    let _ = app_state.first_actor.notify(msg).await;
}


#[cfg_attr(target_arch = "wasm32", tokio::main(flavor = "current_thread"))]
#[cfg_attr(not(target_arch = "wasm32"), tokio::main(flavor = "multi_thread"))]
async fn main() {
    debug_print!("Starting Rust backend");
    use crate::tokio::sync::mpsc;

    // Create actors and get their addresses
    let first_actor_addr = create_actors().await;

    // Create shared app state
    let app_state = AppState {
        first_actor: first_actor_addr,
        counter: Arc::new(AtomicUsize::new(0)),
        todos: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        next_id: Arc::new(tokio::sync::Mutex::new(1)),
    };

    // Set up router with shared state
    let router = Router::new()
        .route(handle_small_text)
        .route(routes::handle_todo_command)
        .with_state(app_state);

    // Run router in background
    spawn(async move {
        router.run().await;
    });

    debug_print!("Router and actors initialized");

    // Keep the main function running until Dart shutdown.
    dart_shutdown().await;
}


// #[cfg(target_arch = "wasm32")]
// #[tokio::main(flavor = "current_thread")]
// async fn main() { /* ... */ }
// 
// #[cfg(not(target_arch = "wasm32"))]
// #[tokio::main(flavor = "multi_thread")]
// async fn main() { /* ... */ }
