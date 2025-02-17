use winapi::{
    shared::ntdef::{HANDLE, NULL, TRUE},
    um::tlhelp32::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    },
};

pub struct Processes {
    processes_snapshot: HANDLE,
    current_process: PROCESSENTRY32,
    first_iter: bool,
}

impl Processes {
    pub fn new() -> Self {
        let mut new = Self {
            processes_snapshot: NULL,
            current_process: PROCESSENTRY32::default(),
            first_iter: true,
        };

        new.take_snapshot();

        new
    }

    pub fn retake_snapshot(&mut self) {
        self.take_snapshot();
    }

    fn take_snapshot(&mut self) {
        let processes_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
        self.processes_snapshot = processes_snapshot;
    }
}

impl Default for Processes {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Processes {
    type Item = PROCESSENTRY32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iter {
            self.first_iter = false;

            self.current_process.dwSize =
                u32::try_from(std::mem::size_of::<PROCESSENTRY32>()).unwrap_or(0);

            unsafe {
                if Process32First(self.processes_snapshot, &mut self.current_process) == TRUE.into()
                {
                    return Some(self.current_process);
                }
            }
        } else {
            unsafe {
                if Process32Next(self.processes_snapshot, &mut self.current_process) == TRUE.into()
                {
                    return Some(self.current_process);
                }
            }
        }

        None
    }
}
