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

//! Interrupt IN reads as the controller driver answers them: the next
//! report a test queued for the endpoint's DCI, else none pending. Each ask
//! is counted, as each is an IPC round trip to driver.xhci0 in the driver.

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use super::XhciClientError;

thread_local! {
    static QUEUED: RefCell<VecDeque<(u8, Vec<u8>)>> = const { RefCell::new(VecDeque::new()) };
    static ASKS: Cell<u32> = const { Cell::new(0) };
}

/// Nothing queued, nothing asked.
pub fn clear() {
    QUEUED.with(|q| q.borrow_mut().clear());
    ASKS.with(|a| a.set(0));
}

/// The device on `dci` has `report` ready, after any queued before it.
pub fn queue(dci: u8, report: &[u8]) {
    QUEUED.with(|q| q.borrow_mut().push_back((dci, report.to_vec())));
}

/// The reads asked since the last `clear`.
pub fn asks() -> u32 {
    ASKS.with(Cell::get)
}

pub fn interrupt_in(
    _xhci_port: u32,
    _slot: u8,
    dci: u8,
    length: u16,
    out: &mut [u8],
) -> Result<Option<usize>, XhciClientError> {
    ASKS.with(|a| a.set(a.get() + 1));
    let next = QUEUED.with(|q| {
        let mut q = q.borrow_mut();
        let i = q.iter().position(|(d, _)| *d == dci)?;
        q.remove(i).map(|(_, r)| r)
    });
    let Some(report) = next else { return Ok(None) };
    let n = report.len().min(usize::from(length)).min(out.len());
    out[..n].copy_from_slice(&report[..n]);
    Ok(Some(n))
}
