#[test]
fn processes() {
    use crate::process::Processes;
    use std::str;

    // Find the "explorer.exe" process
    _ = Processes::new()
        .find(|process| {
            let transmuted = unsafe { &*(&process.szExeFile as *const [i8] as *const [u8]) };
            let string = str::from_utf8(transmuted).unwrap();
            string.starts_with("explorer.exe")
        })
        .expect("Explorer process doesn't exist");
}

#[test]
fn create_injector() {
    use crate::Injector;

    _ = Injector::new()
        .process_name("explorer.exe")
        .dll_path("C:\\Path\\To\\Dll");
}
