// NONOS Operating System (AGPL-3.0-or-later)
//! The bulk calls by pair, as the driver's UAS transport makes them: each
//! recorded with its pair, then answered as the first pair's are.

use super::{bulk_in, bulk_out, reset_bulk, DEVICE};

pub const DATA_PAIR: u8 = 1;

pub fn bulk_out_on(xhci: u32, slot: u8, pair: u8, data: &[u8]) -> Result<usize, i32> {
    DEVICE.with(|d| d.borrow_mut().sent.push((pair, data.to_vec())));
    bulk_out(xhci, slot, data)
}

pub fn bulk_in_on(xhci: u32, slot: u8, pair: u8, out: &mut [u8]) -> Result<usize, i32> {
    DEVICE.with(|d| d.borrow_mut().read_on.push(pair));
    bulk_in(xhci, slot, out)
}

pub fn reset_bulk_on(xhci: u32, slot: u8, _pair: u8, dir_in: bool) -> Result<(), i32> {
    reset_bulk(xhci, slot, dir_in)
}
