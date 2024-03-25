use actix_web::dev::ServerHandle;

use tokio::sync::Mutex;

#[derive(Default)]
pub struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    pub(crate) async fn register(&self, handle: ServerHandle) {
        *self.inner.lock().await = Some(handle);
    }

    pub(crate) async fn stop(&self) {
        let _ = self.inner.lock().await.as_ref().unwrap().stop(true);
    }
}

