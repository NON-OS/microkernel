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

//! Bringing every VMD's domain up, once, and numbering their segments.

extern crate alloc;

use alloc::vec::Vec;

use super::super::types::PciDevice;
use super::bring_up::bring_up;
use super::domain::is_intel_vmd;
use super::registry::{DOMAINS, SEGMENT_BASE};
use crate::sys::serial::Line;

/// Bring up every VMD among `segment0`. Idempotent: the first call decides.
pub(super) fn ensure(segment0: &[PciDevice]) {
    DOMAINS.call_once(|| {
        let mut out = Vec::new();
        let found = segment0.iter().filter(|d| is_intel_vmd(d.vendor_id, d.device_id)).count();
        if found == 0 {
            // One line either way, so a photo shows the probe ran and what
            // it saw: here, the disks are on the root bus.
            Line::new().str(b"[VMD] no Intel VMD on segment 0; disks are on the root bus").end();
        }
        for dev in segment0.iter().filter(|d| is_intel_vmd(d.vendor_id, d.device_id)) {
            let segment = SEGMENT_BASE + out.len() as u16;
            if let Some(domain) = bring_up(dev, segment) {
                out.push(domain);
            }
        }
        out
    });
}
