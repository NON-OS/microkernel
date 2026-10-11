// NONOS Operating System (AGPL-3.0-or-later)
//! ORDER_REFUND as the meter answers it, set by the test.

use std::cell::Cell;

thread_local! {
    pub static REFUND: Cell<Option<([u8; 20], [u8; 32])>> = const { Cell::new(None) };
}

pub fn order_refund(_order_id: &[u8; 32]) -> Option<([u8; 20], [u8; 32])> {
    REFUND.with(Cell::get)
}
