use std::{thread, time};
use std::future::Future;
use tokio::runtime;
use tokio::task::JoinHandle;
use rezasm_core::util::error::EzasmError;

type OutputType = Result<(), EzasmError>;

pub struct Runtime {
    runtime: runtime::Runtime,
    handle: Option<JoinHandle<OutputType>>,
    pub force_stop: bool,
}

impl Runtime {
    pub fn new() -> Runtime {
        let rt = runtime::Builder::new_current_thread().enable_all().build().unwrap();
        Runtime {
            runtime: rt,
            handle: None,
            force_stop: false,
        }
    }

    pub fn from_rt(rt: runtime::Runtime) -> Runtime {
        Runtime {
            runtime: rt,
            handle: None,
            force_stop: false,
        }
    }

    pub fn call(&mut self, future: impl Future<Output=OutputType> + Sync + Send + Sized + 'static) {
        self.force_stop = false;
        self.handle = Some(self.runtime.spawn(future));
    }

    pub fn is_running(&self) -> bool {
        match &self.handle {
            None => false,
            Some(handle) => handle.is_finished(),
        }
    }

    pub fn abort(&mut self) {
        self.force_stop = true;
        thread::sleep(time::Duration::from_millis(50));

        match &self.handle {
            None => {},
            Some(handle) => {
                if !handle.is_finished() {
                    handle.abort();
                }
            },
        }
    }
}
