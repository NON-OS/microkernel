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

//! The switch-over itself (intel_ports_plan.rs says which writes): after the
//! legacy handoff and before the ports are powered or scanned (sequence.rs),
//! and back on teardown while the claim is still held (teardown.rs). Each
//! write is the broker's masked form, naming only the bits it changes. A
//! refusal or a read-back that differs is said once and the driver goes on
//! with whatever ports are already its own.

use alloc::format;

use nonos_libc::{mk_debug, mk_pci_config_read, mk_pci_config_write};

use super::intel_ports_plan::{restore_steps, switch_steps, took, wants, Masks};
use super::intel_ports_plan::{SUBSYSTEM, USB2PRM, USB3PRM};

/// A switch that took, to be handed back.
#[derive(Clone, Copy, Debug)]
pub struct Switched {
    device_id: u64,
    epoch: u64,
    masks: Masks,
}

pub fn switch(device_id: u64, epoch: u64, vendor: u16) -> Option<Switched> {
    let read = |at: u32, width: u32| mk_pci_config_read(device_id, epoch, at, width);
    let (sub, usb3, usb2) = (read(SUBSYSTEM, 4), read(USB3PRM, 2), read(USB2PRM, 2));
    if sub < 0 || usb3 < 0 || usb2 < 0 {
        return None;
    }
    let masks = Masks { usb3: usb3 as u16, usb2: usb2 as u16 };
    if !wants(vendor, (sub as u16, (sub >> 16) as u16), masks) {
        return None;
    }
    let (mut moved, mut problems) = (false, false);
    // A mask of 0 moves nothing; written, it would be the unmasked form.
    for (at, mask, value) in switch_steps(masks).into_iter().filter(|s| s.1 != 0) {
        let rc = mk_pci_config_write(device_id, epoch, at | (mask as u32) << 16, value);
        let ok = rc >= 0 && took(read(at, 2) as u16, mask, value);
        (moved, problems) = (moved || rc >= 0, problems || !ok);
    }
    let how = if problems { " (refused or not taken; carrying on)" } else { "" };
    say(&format!("intel port switch: usb3 {:#06x} usb2 {:#06x}{how}", masks.usb3, masks.usb2));
    moved.then_some(Switched { device_id, epoch, masks })
}

impl Switched {
    /// The ports back to firmware's routing; nothing to say if the broker refuses.
    pub fn restore(&self) {
        for (at, mask, value) in restore_steps(self.masks).into_iter().filter(|s| s.1 != 0) {
            let _ =
                mk_pci_config_write(self.device_id, self.epoch, at | (mask as u32) << 16, value);
        }
    }
}

fn say(line: &str) {
    let line = format!("[driver_xhci] {line}\n");
    let _ = mk_debug(line.as_ptr(), line.len());
}
