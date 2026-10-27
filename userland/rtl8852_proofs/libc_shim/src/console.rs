// NONOS Operating System (AGPL-3.0-or-later)

//! The boot console and the exit, as the proofs see them: every line the
//! driver writes is kept, one entry per write, so a proof can read what a
//! person at the serial port would (and what must never be there).

use std::cell::RefCell;

thread_local! {
    static LINES: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// One write to the console: kept whole, as the kernel would print it.
pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver passes `len` readable bytes of its own line.
    let line = unsafe { core::slice::from_raw_parts(buf, len) }.to_vec();
    LINES.with(|l| l.borrow_mut().push(line));
    0
}

/// Every write this thread made, oldest first; the log is emptied.
pub fn take_lines() -> Vec<Vec<u8>> {
    LINES.with(|l| core::mem::take(&mut *l.borrow_mut()))
}

/// A capsule exit: in the proofs, a panic that names the code.
pub fn mk_exit(code: i32) -> ! {
    panic!("capsule exit {code}")
}
