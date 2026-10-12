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

use super::report::Report;
use crate::xhci::{hosts, Hosts};

/// How long a USB host is waited for before no device is assumed; how
/// long after the first is found a device is looked for; how long a port is given
/// to report its device; how often ports are looked at after that.
const XHCI_WAIT_MS: u64 = 30_000;
pub(super) const WINDOW_MS: u64 = 10_000;
pub(super) const SETTLE_MS: u64 = 1_500;
pub const RESCAN_MS: u64 = 5_000;

pub struct Scanner {
    /// Every host, each with its root ports' tries.
    pub(super) hosts: Hosts<[u8; 256]>,
    /// How many hosts have registered.
    pub(super) known: usize,
    pub(super) window: Deadline,
    pub(super) settle: Option<Deadline>,
    /// What the last pass found, which the kernel asks for (OP_GET_STATE).
    pub report: Report,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            hosts: hosts(|| [0; 256]),
            known: 0,
            window: Deadline::after_ms(XHCI_WAIT_MS),
            settle: None,
            report: Report::new(),
        }
    }
}
