use std::{mem, thread, time};
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
        self.handle = Some(self.runtime.spawn(future));
    }

    pub fn is_running(&self) -> bool {
        match &self.handle {
            None => false,
            Some(handle) => handle.is_finished(),
        }
    }

    pub fn return_value(&mut self) -> Option<OutputType> {
        match &self.handle {
            None => None,
            Some(_) => self.runtime.block_on(async {
                let x = mem::replace(&mut self.handle, None);
                Some(match x.unwrap().await {
                    Ok(result) => result,
                    Err(_) => Err(EzasmError::TimeoutError()),
                })
            })
        }
    }

    pub fn abort(&mut self) -> Option<OutputType> {
        self.force_stop = true;
        thread::sleep(time::Duration::from_millis(50));
        self.force_stop = false;

        match &self.handle {
            None => {},
            Some(handle) => {
                if !handle.is_finished() {
                    handle.abort();
                }
            },
        }

        self.return_value()
    }
}
