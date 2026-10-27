// NONOS Operating System (AGPL-3.0-or-later)

//! `MkInstallSource` as the kernel serves it: each slot a proof fills is
//! read by offset, a call capped at what the caller asked, zero at the
//! end, `-2` (ENOENT) for a slot the loader found no file for. And the
//! console: every `mk_debug` line kept, for the proofs that read them.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static SLOTS: RefCell<HashMap<u64, Vec<u8>>> = RefCell::new(HashMap::new());
    static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// The bytes the loader left in slot `kind`, or none.
pub fn set_slot(kind: u64, bytes: Option<Vec<u8>>) {
    SLOTS.with(|s| match bytes {
        Some(b) => s.borrow_mut().insert(kind, b),
        None => s.borrow_mut().remove(&kind),
    });
}

pub fn mk_install_source(kind: u64, offset: u64, out: &mut [u8]) -> i64 {
    SLOTS.with(|s| {
        let s = s.borrow();
        let Some(b) = s.get(&kind) else { return -2 };
        let at = (offset as usize).min(b.len());
        let n = out.len().min(b.len() - at);
        out[..n].copy_from_slice(&b[at..at + n]);
        n as i64
    })
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver passes its own line of `len` bytes.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    LINES.with(|l| l.borrow_mut().push(String::from_utf8_lossy(bytes).into_owned()));
    len as i64
}

/// Every console line so far, and the list emptied.
pub fn take_lines() -> Vec<String> {
    LINES.with(|l| std::mem::take(&mut *l.borrow_mut()))
}
