use tokio::runtime;
use tokio::task::JoinHandle;

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

type OutputType = Result<(), String>;

pub struct Runtime {
    runtime: runtime::Runtime,
    handle: Option<JoinHandle<OutputType>>,
    force_stop: Arc<AtomicBool>,
}

impl Runtime {
    pub fn new(force_stop: Arc<AtomicBool>) -> Runtime {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        Runtime {
            runtime: rt,
            handle: None,
            force_stop,
        }
    }

    pub fn from_rt(force_stop: Arc<AtomicBool>, rt: runtime::Runtime) -> Runtime {
        Runtime {
            runtime: rt,
            handle: None,
            force_stop,
        }
    }

    pub fn force_stop(&self) -> bool {
        self.force_stop.load(Ordering::SeqCst)
    }

    pub fn call(
        &mut self,
        future: impl Future<Output = OutputType> + Sync + Send + Sized + 'static,
    ) {
        self.force_stop.store(false, Ordering::SeqCst);
        self.handle = Some(self.runtime.spawn(future));
    }

    pub fn is_running(&self) -> bool {
        match &self.handle {
            None => false,
            Some(handle) => !handle.is_finished(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn sleep(&self, _milliseconds: usize) {}

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep(&self, milliseconds: u64) {
        thread::sleep(time::Duration::from_millis(milliseconds));
    }

    pub fn abort(&mut self) {
        if !self.is_running() {
            return;
        }

        self.force_stop.store(false, Ordering::SeqCst);
        self.sleep(50);

        match &self.handle {
            None => {}
            Some(handle) => {
                if !handle.is_finished() {
                    handle.abort();
                }
            }
        }
    }
}
