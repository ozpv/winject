#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

#[cfg(feature = "async")]
pub mod r#async;

pub mod inject;
pub mod injectors;
pub mod process;

#[cfg(all(test, windows))]
mod tests;

pub mod winject;
#[allow(unused)]
pub use winject::*;

/* Example
#[tokio::main]
async fn main() {
    // build an injector
    let injector = Injector::new()
        .process_name("process_name_here.exe")
        .dll_path("C:\\path\\to\\dll");

    // inject into a process by blocking this thread
    <Injector as TryInject>::try_inject(&injector).unwrap();

    // inject into processes concurrently
    let fut0 = <Injector as AsyncTryInject>::try_inject(&injector);
    let fut1 = <Injector as AsyncTryInject>::try_inject(&injector);

    tokio::try_join!(fut0, fut1).unwrap();

    // do something while injecting
    let fut2 = <Injector as AsyncTryInject>::try_inject(&injector);
    tokio::pin!(fut2);

    loop {
        tokio::select! {
            _ = &mut fut2 => {
                break;
            },
            _ = std::future::ready(()) => {
                println!("Doing work");
            },
        }
    }

    // build an injector with the dll included in this binary
    // this could also be any byte array/Vec of a dll
    let bytes = include_bytes!("main.rs");

    let injector = Injector::new()
        .process_name("something.exe")
        .dll_bytes(bytes);

    <Injector as TryInject>::try_inject(&injector).unwrap();
}
*/
