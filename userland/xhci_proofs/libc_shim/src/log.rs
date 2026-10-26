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

//! The order the driver gave things back in. A test starts the log, may set
//! a probe that reads the device's registers, and each broker call made on
//! its thread is recorded with what the probe saw at that moment: so a test
//! can say the controller was halted when bus mastering went off, and that
//! the device was released before any DMA frame was.

use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    /// A write of the PCI Command register, with the value written.
    PciCommand(u16),
    /// A masked write tried elsewhere in config space: register, mask, value.
    PciConfig(u32, u16, u16),
    IrqUnbind(u64),
    MmioUnmap(u64),
    DeviceRelease(u64),
    DmaUnmap(u64),
}

#[derive(Clone, Copy, Debug)]
pub struct Logged {
    pub call: Call,
    /// What the probe read when the call was made, 0 with no probe.
    pub seen: u64,
}

type Probe = Box<dyn Fn() -> u64>;

thread_local! {
    static LOG: RefCell<Option<Vec<Logged>>> = const { RefCell::new(None) };
    static PROBE: RefCell<Option<Probe>> = const { RefCell::new(None) };
}

/// Record this thread's broker calls from now on, `probe` read at each.
pub fn start_log(probe: impl Fn() -> u64 + 'static) {
    PROBE.with(|p| *p.borrow_mut() = Some(Box::new(probe)));
    LOG.with(|l| *l.borrow_mut() = Some(Vec::new()));
}

/// The calls recorded so far, and recording stops.
pub fn take_log() -> Vec<Logged> {
    PROBE.with(|p| *p.borrow_mut() = None);
    LOG.with(|l| l.borrow_mut().take().unwrap_or_default())
}

pub(crate) fn record(call: Call) {
    let seen = PROBE.with(|p| p.borrow().as_ref().map_or(0, |f| f()));
    LOG.with(|l| {
        if let Some(log) = l.borrow_mut().as_mut() {
            log.push(Logged { call, seen });
        }
    });
}
