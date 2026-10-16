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

//! The register and interrupt grants a bring-up asks for. Memory BARs are
//! refused (the tests drive the legacy window through ports); a port grant
//! is given; the interrupt kinds the test offers are bound.

use std::cell::Cell;

use crate::consts::MK_IRQ_BIND_MSIX;
use crate::events::{record, Event};
use crate::types::{irq::IrqBindOut, mmio::MmioMapOut, pio::PioGrantOut};

thread_local! {
    static OFFER: Cell<(bool, bool)> = const { Cell::new((true, true)) };
}

/// Which interrupts the next binds get: MSI-X, and the INTx line.
pub fn offer_irq(msix: bool, intx: bool) {
    OFFER.with(|o| o.set((msix, intx)));
}

pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    _bar: u32,
    _f: u32,
    _o: u64,
    _l: u64,
    _out: *mut MmioMapOut,
) -> i64 {
    record(Event::MmioMap);
    -19
}

pub fn mk_mmio_unmap(_grant: u64) -> i64 {
    0
}

pub fn mk_pio_grant(_d: u64, _e: u64, _bar: u8, _flags: u32, out: *mut PioGrantOut) -> i64 {
    record(Event::PioGrant);
    // SAFETY: the driver passes the address of one of its own records.
    unsafe { (*out).grant_id = 5 };
    0
}

pub fn mk_irq_bind(_d: u64, _e: u64, _src: u32, flags: u32, _n: u32, out: *mut IrqBindOut) -> i64 {
    let msix = flags & MK_IRQ_BIND_MSIX != 0;
    record(Event::IrqBind { msix });
    let (offers_msix, offers_intx) = OFFER.with(Cell::get);
    if !(if msix { offers_msix } else { offers_intx }) {
        return -95;
    }
    // SAFETY: the driver passes the address of one of its own records.
    unsafe { (*out).grant_id = 3 };
    0
}
