use std::sync::Arc;
use tokio::sync::{Notify, Mutex};

pub struct AIOEvent {
    notify: Arc<Notify>,
    is_on: Arc<Mutex<bool>>
}

impl AIOEvent {

    pub fn new() -> AIOEvent {
        let notify = Arc::new(Notify::new());
        AIOEvent {
            notify,
            is_on: Arc::new(Mutex::new(false))
        }
    }

    pub async fn wait(&self) {
        if self.is_on.lock().await.to_owned() { return; }
        self.notify.notified().await;
    }
    
    pub async fn set(&mut self) {
        *self.is_on.lock().await = true;
        self.notify.notify_waiters();
    }

    pub async fn clear(&mut self) {
        *self.is_on.lock().await = false;
    }

    pub async fn is_set(&mut self) -> bool {
        self.is_on.lock().await.to_owned()
    }

}

impl Clone for AIOEvent {
    fn clone(&self) -> AIOEvent {   
        AIOEvent {notify: self.notify.clone(), is_on: self.is_on.clone()}
    }
}