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
//! The walk over the device list that collects the candidates.

use alloc::vec;

use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_PCI};

use super::candidate::is_candidate;
use super::found::found;
use super::machine::{Survey, MAX_CONTROLLERS};
use crate::constants::CLASS_AUDIO;
use crate::controller::intel::{amd_acp, graphics_audio};
use crate::controller::sst::intel_sst;

const MAX_DEVICES: usize = 128;
/// Every class the broker reports; the ACP is filed under "other".
const CLASS_ALL: u32 = 0;

pub fn survey() -> Survey {
    let mut s = Survey { hda: [None; MAX_CONTROLLERS], amd_acp: false, intel_sst: None };
    let mut buf = vec![DeviceRecord::empty(); MAX_DEVICES];
    let mut n = 0usize;
    // Chipset controllers first, graphics after.
    for graphics in [false, true] {
        let got = mk_device_list(CLASS_AUDIO, buf.as_mut_ptr(), MAX_DEVICES as u64);
        for r in &buf[..clamp(got)] {
            if n < MAX_CONTROLLERS
                && is_candidate(r)
                && graphics_audio(r.vendor, r.device) == graphics
            {
                s.hda[n] = Some(found(r));
                n += 1;
            }
        }
    }
    let got = mk_device_list(CLASS_ALL, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let pci = buf[..clamp(got)].iter().filter(|r| r.bus_kind == BUS_KIND_PCI);
    for r in pci {
        s.amd_acp |= amd_acp(r.vendor, r.pci_class, r.pci_subclass);
        if intel_sst(r.vendor, r.device) {
            s.intel_sst = Some(r.device);
        }
    }
    s
}

fn clamp(got: i64) -> usize {
    if got <= 0 {
        0
    } else {
        core::cmp::min(got as usize, MAX_DEVICES)
    }
}
