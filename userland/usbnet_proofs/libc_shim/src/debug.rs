// NONOS Operating System (AGPL-3.0-or-later)
//! The serial log: every line the core writes, kept for the proof.

use std::cell::RefCell;

thread_local! {
    static LINES: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() {
        return -14;
    }
    // SAFETY: the core passes a pointer to one of its own buffers and the
    // number of bytes it wrote there.
    let line = unsafe { core::slice::from_raw_parts(buf, len) }.to_vec();
    LINES.with(|l| l.borrow_mut().push(line));
    len as i64
}

/// Every line written on this thread so far, newline included.
pub fn lines() -> Vec<Vec<u8>> {
    LINES.with(|l| l.borrow().clone())
}
