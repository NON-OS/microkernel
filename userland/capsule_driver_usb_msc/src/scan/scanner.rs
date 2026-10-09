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

//! Looking for the device. Nothing tells the kernel that a USB
//! mass-storage device is plugged in until the controller driver has
//! enumerated its ports, so this driver starts wherever an xHCI controller
//! is and decides here, within a bounded window, whether it has a device.
//!
//! The window ran from this driver's start. On a laptop whose controller
//! takes seconds to come up, and a stick that takes seconds more to say it
//! is ready, it could close before the stick was ever asked, and the boot
//! went on with no store. It now runs from when the controller is found,
//! and once it closes, ports are still looked at every few seconds: a port
//! newly connected, or one that failed, is tried again, so a late stick is
//! still taken.

use nonos_libc::Deadline;

use super::medium::Medium;
use super::pass::{pass, NOT_OURS};
use super::report::{Report, Stage};
use crate::state::State;
use crate::xhci::{connected_ports, lookup};

/// How long driver.xhci0 is waited for before no device is assumed; how
/// long after it is found a device is looked for; how long a port is given
/// to report its device; how often ports are looked at after that.
const XHCI_WAIT_MS: u64 = 30_000;
const WINDOW_MS: u64 = 10_000;
const SETTLE_MS: u64 = 1_500;
pub const RESCAN_MS: u64 = 5_000;

pub struct Scanner {
    xhci: Option<u32>,
    window: Deadline,
    settle: Option<Deadline>,
    tries: [u8; 256],
    /// What the last pass found, which the kernel asks for (OP_GET_STATE).
    pub report: Report,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            xhci: None,
            window: Deadline::after_ms(XHCI_WAIT_MS),
            settle: None,
            tries: [0; 256],
            report: Report::new(),
        }
    }

    /// One pass over the ports. `Scanning` until the device is bound or
    /// every connected port is decided, or the window closes.
    pub fn step(&mut self, state: &mut State) -> Medium {
        let Some(xhci) = self.found() else { return self.undecided() };
        let settle = *self.settle.get_or_insert_with(|| Deadline::after_ms(SETTLE_MS));
        let ports = match connected_ports(xhci) {
            Ok(ports) => ports,
            Err(e) => {
                self.report = Report::at(Stage::NoXhci, 0, e);
                return self.undecided();
            }
        };
        let (found, open) = pass(xhci, ports, &mut self.tries, state, &mut self.report);
        if let Some(disk) = found {
            return Medium::Ready(disk);
        }
        if open || !settle.expired() {
            return self.undecided();
        }
        Medium::Absent
    }

    /// After the window: every port that failed is given its tries again,
    /// and one pass looks at every port not of another class.
    pub fn again(&mut self, state: &mut State) -> Medium {
        let Some(xhci) = self.found() else { return Medium::Absent };
        for t in self.tries.iter_mut().filter(|t| **t != NOT_OURS) {
            *t = 0;
        }
        let Ok(ports) = connected_ports(xhci) else { return Medium::Absent };
        match pass(xhci, ports, &mut self.tries, state, &mut self.report) {
            (Some(disk), _) => Medium::Ready(disk),
            _ => Medium::Absent,
        }
    }

    /// driver.xhci0, once it has registered; the window restarts then.
    fn found(&mut self) -> Option<u32> {
        if let Some(x) = self.xhci {
            return Some(x);
        }
        let Some(x) = lookup() else {
            self.report = Report::new();
            return None;
        };
        self.xhci = Some(x);
        self.window = Deadline::after_ms(WINDOW_MS);
        Some(x)
    }

    fn undecided(&self) -> Medium {
        if self.window.expired() {
            Medium::Absent
        } else {
            Medium::Scanning
        }
    }
}
