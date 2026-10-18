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

//! Reset a claimed function at claim and at release (K10a), with Bus
//! Master already clear: whatever engines a dead driver, a predecessor or
//! firmware left armed stop before a frame is freed or a new driver gets
//! the device. A display device is left as it is: the firmware's scanout,
//! which the boot console draws on, lives in it.

extern crate alloc;

use spin::Mutex;

use super::reset_function::Function;
use super::reset_rule::Reset;
use super::reset_run::reset;
use crate::drivers::pci::config::ConfigSpace;

const CLASS_DISPLAY: u8 = 0x03;

/// Devices already said to offer no reset, so each is said once.
static UNRESET: Mutex<alloc::vec::Vec<u64>> = Mutex::new(alloc::vec::Vec::new());

/// At claim, from the claimant's own MkDeviceClaim, whose waits may sleep.
pub(super) fn at_claim(device_id: u64, pid: u32) {
    run(device_id, Some(pid), b" not reset at claim: no FLR");
}

/// At release; `sleeper` names a caller in its own MkDeviceRelease.
pub(super) fn at_release(device_id: u64, sleeper: Option<u32>) {
    run(device_id, sleeper, b" not reset at release: no FLR");
}

fn run(device_id: u64, sleeper: Option<u32>, unoffered: &[u8]) {
    let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) else {
        return;
    };
    let cfg = ConfigSpace::new(handle.address);
    if matches!(cfg.class_code(), Ok(CLASS_DISPLAY)) {
        return;
    }
    let said: &[u8] = match reset(&mut Function::new(cfg, sleeper)) {
        Reset::Done { pending_left: false } => return,
        Reset::Done { pending_left: true } => b" reset, transactions still pending after 700 ms",
        Reset::NotReady => b" reset, then silent for 1000 ms: header not restored",
        Reset::NotOffered => {
            let mut seen = UNRESET.lock();
            if seen.contains(&device_id) {
                return;
            }
            seen.push(device_id);
            unoffered
        }
    };
    crate::sys::serial::print(b"[BROKER] device ");
    crate::sys::serial::print_hex(device_id);
    crate::sys::serial::println(said);
}
