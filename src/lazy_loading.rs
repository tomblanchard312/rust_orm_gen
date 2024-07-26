use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LazyLoaded<T> {
    value: Arc<Mutex<Option<T>>>,
    loader: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> LazyLoaded<T> {
    pub fn new<F>(loader: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        LazyLoaded {
            value: Arc::new(Mutex::new(None)),
            loader: Box::new(loader),
        }
    }

    pub async fn get(&self) -> T
    where
        T: Clone,
    {
        let mut value = self.value.lock().await;
        if value.is_none() {
            *value = Some((self.loader)());
        }
        value.as_ref().unwrap().clone()
    }
}