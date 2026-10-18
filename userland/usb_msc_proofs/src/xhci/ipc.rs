// NONOS Operating System (AGPL-3.0-or-later)
//! driver.xhci0's end of the IPC call the driver's own xhci/call.rs makes,
//! for the slot requests its xhci/slot.rs sends: each is read off the wire
//! and recorded, the scripted device takes its time on the test clock, and
//! the reply is framed as the controller frames it. A call that would
//! outlast its timeout fails -110 there, as the kernel's does.

use super::device::setup;
use super::window::cost;
use super::wire::{DATA_AT, HDR_LEN, MAGIC, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_ENABLE_SLOT};
use crate::clock::advance;
use crate::protocol::{E_INVAL, E_IO, E_TIMEDOUT};

/// nonos_libc's call, as the driver makes it (the crate root names this
/// crate nonos_libc for it).
pub(crate) fn mk_ipc_call_timeout(
    _endpoint: u64,
    req: *const u8,
    req_len: usize,
    resp: *mut u8,
    resp_len: usize,
    timeout_ms: u64,
) -> i64 {
    // SAFETY: xhci/call.rs passes its request buffer with the length of the
    // request in it, and its whole reply buffer; both are its own locals,
    // live across the call, and do not overlap.
    let (req, resp) = unsafe {
        (core::slice::from_raw_parts(req, req_len), core::slice::from_raw_parts_mut(resp, resp_len))
    };
    let answer = asked(u16::from_le_bytes([req[6], req[7]]), &req[HDR_LEN..]);
    let (ms, answered) = cost();
    if ms > timeout_ms {
        advance(timeout_ms);
        return i64::from(E_TIMEDOUT);
    }
    advance(ms);
    let (status, data) = if answered { answer } else { (E_IO, vec![]) };
    resp[..4].copy_from_slice(&MAGIC.to_le_bytes());
    resp[12..16].copy_from_slice(&req[12..16]);
    resp[16..20].copy_from_slice(&(4 + data.len() as u32).to_le_bytes());
    resp[HDR_LEN..DATA_AT].copy_from_slice(&status.to_le_bytes());
    resp[DATA_AT..DATA_AT + data.len()].copy_from_slice(&data);
    (DATA_AT + data.len()) as i64
}

/// Request `op` with `body`, recorded, and the controller's answer to it:
/// slot 1, an address on it, and a slot given back.
fn asked(op: u16, body: &[u8]) -> (i32, Vec<u8>) {
    setup(|d| match (op, body) {
        (OP_ENABLE_SLOT, []) => {
            d.enabled += 1;
            (0, vec![1, 0, 0, 0])
        }
        (OP_ADDRESS_DEVICE, &[slot, port]) => {
            d.addressed.push(port);
            (0, vec![slot, port, 0, 0, 0, 0, 0, 0])
        }
        (OP_DISABLE_SLOT, &[slot]) => {
            d.disabled.push(slot);
            (0, vec![])
        }
        _ => (E_INVAL, vec![]),
    })
}
