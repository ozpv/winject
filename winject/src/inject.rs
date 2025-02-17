use thiserror::Error;

pub trait TryInject {
    type Result;

    fn try_inject(&self) -> Self::Result;
}

pub trait TryLoadBytes {
    type Result;

    fn try_load_bytes(&mut self) -> Self::Result;
}

#[derive(Debug, Error)]
pub enum InjectError {
    #[error("Must specify a dll to inject")]
    MissingDll,
    #[error("Must specify a process to inject into")]
    MissingProcess,
}
