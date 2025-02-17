use std::future::Future;

pub trait AsyncTryInject {
    type Error;

    fn try_inject(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub trait AsyncTryLoadBytes {
    fn try_load_bytes(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;
}

impl AsyncTryInject for Injector {
    type Error = InjectError;

    /// Async version of `try_inject`
    ///
    /// # Errors
    ///
    /// If `dll_path` fails open or read
    async fn try_inject(&self) -> Result<(), Self::Error> {
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

impl AsyncTryLoadBytes for Injector {
    /// Async version of `try_load_bytes`
    ///
    /// # Errors
    ///
    /// If `dll_path` fails open or read
    async fn try_load_bytes(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
