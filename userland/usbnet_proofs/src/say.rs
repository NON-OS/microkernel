// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's log lines on the host: kept, one per call on this thread,
//! so a proof can count what a bind said.

use std::cell::RefCell;

thread_local! {
    static SAID: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// `[usbnet <tag>] <parts...>`, as the capsule's line reads.
pub fn say(tag: &[u8], parts: &[&[u8]]) {
    let mut line = format!("[usbnet {}] ", String::from_utf8_lossy(tag));
    parts.iter().for_each(|p| line.push_str(&String::from_utf8_lossy(p)));
    SAID.with(|s| s.borrow_mut().push(line));
}

/// Every line said on this thread so far, and forgotten.
pub fn said() -> Vec<String> {
    SAID.with(|s| s.take())
}
