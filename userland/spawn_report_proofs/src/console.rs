// NONOS Operating System (AGPL-3.0-or-later)
//! The serial console and the boot log as one buffer, so a test reads back
//! exactly the lines the report printed.

use std::sync::Mutex;

pub static OUT: Mutex<String> = Mutex::new(String::new());

/// Everything written so far, and the buffer emptied.
pub fn take() -> String {
    core::mem::take(&mut *OUT.lock().unwrap())
}

fn put(s: &str) {
    OUT.lock().unwrap().push_str(s);
}

pub mod serial {
    /// The kernel's line builder, reduced to what the report uses: the whole
    /// line reaches the buffer in one write, newline included.
    #[derive(Default)]
    pub struct Line(String);

    impl Line {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn str(&mut self, b: &[u8]) -> &mut Self {
            self.0.push_str(&String::from_utf8_lossy(b));
            self
        }
        pub fn dec(&mut self, v: u64) -> &mut Self {
            self.0.push_str(&v.to_string());
            self
        }
        pub fn end(&self) {
            super::put(&format!("{}\n", self.0));
        }
    }
}

pub mod boot_log {
    pub fn ok(tag: &str, msg: &str) {
        super::put(&format!("[{tag}] {msg}\n"));
    }
    pub fn error(msg: &str) {
        super::put(&format!("[ERROR] {msg}\n"));
    }
}
