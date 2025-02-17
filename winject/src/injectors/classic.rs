use std::path::PathBuf;
use winapi::shared::minwindef::DWORD;

use crate::inject::{InjectError, TryInject, TryLoadBytes};

/// Injector using the classic Dll injector method
#[derive(Clone, Debug)]
pub struct ClassicInjector {
    process_name: Option<String>,
    process_id: Option<DWORD>,
    dll_path: Option<PathBuf>,
    dll_bytes: Option<Vec<u8>>,
}

impl ClassicInjector {
    pub fn new() -> Self {
        Self {
            process_name: None,
            process_id: None,
            dll_path: None,
            dll_bytes: None,
        }
    }

    pub fn process_name(mut self, process_name: impl Into<String>) -> Self {
        self.process_id = None;
        self.process_name = Some(process_name.into());
        self
    }

    pub fn process_id(mut self, process_id: DWORD) -> Self {
        self.process_name = None;
        self.process_id = Some(process_id);
        self
    }

    pub fn dll_path(mut self, dll_path: impl Into<PathBuf>) -> Self {
        self.dll_bytes = None;
        self.dll_path = Some(dll_path.into());
        self
    }

    pub fn dll_bytes<B: Into<Vec<u8>>>(mut self, dll_bytes: B) -> Self {
        self.dll_path = None;
        self.dll_bytes = Some(dll_bytes.into());
        self
    }

    /// Does the exact opposite of `try_load_bytes`
    /// Could be helpful for long-lived `ClassicInjector`s with ginormous dll sizes
    pub fn unload_bytes(&mut self) {
        self.dll_bytes = None;
    }

    pub fn get_process_name(&self) -> Option<&String> {
        self.process_name.as_ref()
    }

    pub fn get_process_id(&self) -> Option<DWORD> {
        self.process_id
    }

    pub fn get_dll_path(&self) -> Option<&PathBuf> {
        self.dll_path.as_ref()
    }

    pub fn get_dll_bytes(&self) -> Option<&Vec<u8>> {
        self.dll_bytes.as_ref()
    }
}

impl Default for ClassicInjector {
    fn default() -> Self {
        Self::new()
    }
}

impl TryInject for ClassicInjector {
    type Result = Result<(), InjectError>;

    /// Attempts to inject into a process
    ///
    /// # Errors
    ///
    /// If `dll_path` fails open or read
    fn try_inject(&self) -> Self::Result {
        if self.process_id.is_none() && self.process_name.is_none() {
            return Err(InjectError::MissingProcess);
        }

        if self.dll_path.is_none() && self.dll_bytes.is_none() {
            return Err(InjectError::MissingDll);
        }

        println!("Injected");

        Ok(())
    }
}

impl TryLoadBytes for ClassicInjector {
    type Result = std::io::Result<()>;
    /// By default, `dll_path`'s bytes are lazily loaded
    /// meaning that the dll file is opened on an inject
    /// this method allows you to read the dll before injection(s)
    ///
    /// note the dll isn't opened multiple times on multiple injections
    /// so you don't need to worry about anything like that
    ///
    /// # Errors
    ///
    /// If `dll_path` fails open or read
    fn try_load_bytes(&mut self) -> Self::Result {
        todo!()
    }
}
