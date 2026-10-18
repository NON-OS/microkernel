// NONOS Operating System (AGPL-3.0-or-later)
//! The console, recorded so a test can ask what boot reported.

pub mod serial {
    use std::sync::Mutex;

    pub static LINES: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());

    pub fn println(line: &[u8]) {
        LINES.lock().unwrap().push(line.to_vec());
    }
}
