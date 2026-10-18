// NONOS Operating System (AGPL-3.0-or-later)
//! A fake settle port for the step machine's tests: every frame the wallet
//! sends is kept, by op, id and body, and answered by the test, or not.

use std::cell::{Cell, RefCell};

use nonos_payment_proto::EAGAIN;

pub const ESCROW: u8 = 1;
pub const LANDED: u8 = 2;
/// The pid the registry names for settle.
pub const SETTLE_PID: u32 = 42;

thread_local! {
    pub static SENT: RefCell<Vec<(u16, u32, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
    /// The port refuses the send, as when settle is not running.
    pub static DOWN: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn send(op: u16, id: u32, body: &[u8]) -> Result<u32, i32> {
    if DOWN.with(Cell::get) {
        return Err(EAGAIN);
    }
    SENT.with(|s| s.borrow_mut().push((op, id, body.to_vec())));
    Ok(SETTLE_PID)
}

/// The last frame sent.
pub fn last() -> Option<(u16, u32, Vec<u8>)> {
    SENT.with(|s| s.borrow().last().cloned())
}
