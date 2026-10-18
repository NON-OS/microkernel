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

//! PCI_FIRMWARE (22, device id u64): what the PCI `_DSM` says of one PCI
//! function, for the inventory. Little-endian:
//!
//! - boot configuration u8: 0 unsaid, 1 kept, 2 may be redone (its host
//!   bridge's function 5);
//! - readiness present u8 (0 or 1), then five u32 in microseconds: after
//!   reset, after link up, after FLR, D3hot to D0, after VF enable (its
//!   host bridge's function 9);
//! - label instance u64 and label length u8 (0 with no label), then its
//!   bytes (function 7).
//!
//! ENOENT for a device that is no PCI function below a known host bridge;
//! EINVAL for a short request.

use crate::boot::Acpi;
use crate::server::protocol::{EINVAL, ENOENT};
use crate::sys::function_of::{function_of, Asker};
use alloc::vec::Vec;

pub fn pci_firmware(acpi: &Acpi, req: &[u8]) -> (i32, Vec<u8>) {
    let Some(id) = req.get(1..9).and_then(|b| b.try_into().ok()).map(u64::from_le_bytes) else {
        return (EINVAL, Vec::new());
    };
    let Ok(f) = function_of(Asker::Capsule, id) else { return (ENOENT, Vec::new()) };
    let pf = &acpi.pci_firmware;
    let Some(h) = pf.host_of(f) else { return (ENOENT, Vec::new()) };
    let keep = match h.facts.preserve_boot_config {
        None => 0,
        Some(true) => 1,
        Some(false) => 2,
    };
    let mut v = alloc::vec![keep, u8::from(h.facts.readiness.is_some())];
    let r = h.facts.readiness;
    let us = r
        .map_or([0; 5], |r| [r.reset_us, r.link_up_us, r.flr_us, r.d3hot_to_d0_us, r.vf_enable_us]);
    us.iter().for_each(|u| v.extend_from_slice(&u.to_le_bytes()));
    match pf.labels.iter().find(|l| l.bdf == f) {
        Some(l) => {
            v.extend_from_slice(&l.label.instance.to_le_bytes());
            v.push(l.label.text.len() as u8);
            v.extend_from_slice(&l.label.text);
        }
        None => v.extend_from_slice(&[0; 9]),
    }
    (0, v)
}
