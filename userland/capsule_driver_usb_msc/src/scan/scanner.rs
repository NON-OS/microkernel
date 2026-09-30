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

//! Looking for the device at start. Nothing tells the kernel that a USB
//! mass-storage device is plugged in until the controller driver has
//! enumerated its ports, so this driver starts wherever an xHCI controller
//! is and decides here, within a bounded window, whether it has a device.

use nonos_libc::Deadline;

use super::medium::Medium;
use super::pass::pass;
use crate::state::State;
use crate::xhci::{connected_ports, lookup};

/// The longest the search runs, and how long after the controller is
/// found a port is given to report its device before none is assumed.
const WINDOW_MS: u64 = 10_000;
const SETTLE_MS: u64 = 1_500;

pub struct Scanner {
    xhci: Option<u32>,
    window: Deadline,
    settle: Option<Deadline>,
    tries: [u8; 256],
}

impl Scanner {
    pub fn new() -> Self {
        Self { xhci: None, window: Deadline::after_ms(WINDOW_MS), settle: None, tries: [0; 256] }
    }

    /// One pass over the ports. `Scanning` until the device is bound or
    /// every connected port is decided, or the window closes.
    pub fn step(&mut self, state: &mut State) -> Medium {
        let Some(xhci) = self.xhci.or_else(lookup) else { return self.undecided() };
        self.xhci = Some(xhci);
        let settle = *self.settle.get_or_insert_with(|| Deadline::after_ms(SETTLE_MS));
        let Ok(ports) = connected_ports(xhci) else { return self.undecided() };
        let (found, open) = pass(xhci, ports, &mut self.tries, state);
        if let Some(disk) = found {
            return Medium::Ready(disk);
        }
        if open || !settle.expired() {
            return self.undecided();
        }
        Medium::Absent
    }

    fn undecided(&self) -> Medium {
        if self.window.expired() {
            Medium::Absent
        } else {
            Medium::Scanning
        }
    }
}
