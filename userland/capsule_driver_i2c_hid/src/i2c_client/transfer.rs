// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec;

use nonos_libc::mk_ipc_call_timeout;

use crate::i2c_client::{seq, wire};

pub fn write_read(port: u32, addr: u8, write: &[u8], read: &mut [u8]) -> Option<usize> {
    let request_id = seq::next();
    // Request frame: header + the 8-byte transfer body header + the write bytes.
    let mut tx = vec![0u8; wire::HDR_LEN + 8 + write.len()];
    let n = wire::request(request_id, addr, write, read.len(), &mut tx);
    if n == 0 {
        return None;
    }
    // Response frame: the read data lands at a fixed 32-byte offset, so size the
    // reply buffer to the requested read length plus that offset. A fixed cap
    // here silently truncates large reads (the HID report descriptor above all).
    let mut rx = vec![0u8; 32 + read.len()];
    if !super::gate::may_call() {
        return None;
    }
    let timeout = call_timeout(write, read);
    let got = mk_ipc_call_timeout(port as u64, tx.as_ptr(), n, rx.as_mut_ptr(), rx.len(), timeout);
    super::gate::record(got);
    receive(&mut rx, got, request_id, read)
}

/// The wait for a transfer of `write` and then `read`: the controller moves
/// the bytes both ways in the one transfer (budget.rs).
pub fn call_timeout(write: &[u8], read: &[u8]) -> u64 {
    super::budget::call_timeout_ms(write.len() + read.len())
}

/// The read data out of the `got` bytes of reply in `rx`, into `read`; and
/// `rx` cleared, as an input report passed through it on its way to the
/// caller and none of it stays in freed memory (plan/50 C8). black_box
/// keeps the clear from being dropped as a store to a buffer about to be
/// freed.
pub fn receive(rx: &mut [u8], got: i64, request_id: u64, read: &mut [u8]) -> Option<usize> {
    let got = usize::try_from(got).ok().filter(|&g| g <= rx.len());
    let n = got.and_then(|g| wire::response(&rx[..g], request_id, read));
    rx.fill(0);
    core::hint::black_box(&rx);
    n
}
