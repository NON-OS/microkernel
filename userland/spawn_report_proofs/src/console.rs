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
    pub fn print(b: &[u8]) {
        super::put(&String::from_utf8_lossy(b));
    }
    pub fn println(b: &[u8]) {
        print(b);
        super::put("\n");
    }
    pub fn print_dec(v: u64) {
        super::put(&v.to_string());
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
