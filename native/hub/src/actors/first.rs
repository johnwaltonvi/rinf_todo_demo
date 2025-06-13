use std::time::Duration;

use crate::signals::{BigBool, SmallNumber, SmallText};
use crate::signals::{TodoItem, TodoList, AddTodo, ToggleTodo, DeleteTodo, GetTodos};
use async_trait::async_trait;
use messages::prelude::{Actor, Address, Context, Handler, Notifiable};
use rinf::{DartSignal, RustSignal, debug_print};

use crate::tokio::task::JoinSet;
use crate::tokio::time::interval;


/// The first actor.
pub struct FirstActor {
    /// Owned tasks that are canceled when the actor is dropped.
    _owned_tasks: JoinSet<()>,
    // Store reference to shared state if needed
    app_state: Option<crate::AppState>,
}

// Implementing the `Actor` trait for `CountingActor`.
// This defines `FirstActor` as an actor in the async system.
impl Actor for FirstActor {}

impl FirstActor {
    /// Creates the actor and initializes its fields.
    pub fn new(self_addr: Address<Self>) -> Self {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr.clone()));
        _owned_tasks.spawn(Self::listen_to_timer(self_addr));
        FirstActor { 
            _owned_tasks,
            app_state: None,
        }
    }

    // Method to inject app state after creation
    pub fn with_app_state(mut self, app_state: crate::AppState) -> Self {
        self.app_state = Some(app_state);
        self
    }
}

// Implementing the `Notifiable` trait
// allows an actor's loop to listen for a specific message type.
#[async_trait]
impl Notifiable<SmallText> for FirstActor {
    async fn notify(&mut self, msg: SmallText, _: &Context<Self>) {
        debug_print!("{}", msg.text);
        SmallNumber { number: 7 }.send_signal_to_dart();
    }
}

// Note: Todo-related handlers have been moved to route handlers that directly modify AppState

// Implementing the `Handler` trait
// allows an actor's loop to respond to a specific message type.
#[async_trait]
impl Handler<BigBool> for FirstActor {
    type Result = bool;
    async fn handle(&mut self, msg: BigBool, _: &Context<Self>) -> bool {
        msg.send_signal_to_dart();
        false
    }
}

impl FirstActor {
    /// Listen to an external source, which in this case is Dart.
    async fn listen_to_dart(self_addr: Address<Self>) {
        // Create a task for SmallText signal
        let mut small_text_addr = self_addr;
        let small_text_task = async move {
            let receiver = SmallText::get_dart_signal_receiver();
            while let Some(signal_pack) = receiver.recv().await {
                let _ = small_text_addr.notify(signal_pack.message).await;
            }
        };

        // Note: Todo-related signal listeners have been removed as they are now handled by router handlers

        // Run the task
        small_text_task.await;
    }

    /// Listen to an external source, which in this case is a timer.
    async fn listen_to_timer(mut self_addr: Address<Self>) {
        let mut time_interval = interval(Duration::from_secs(3));
        let text = "From an owned task".to_owned();
        loop {
            time_interval.tick().await;
            let _ = self_addr.notify(SmallText { text: text.clone() }).await;
        }
    }
}
