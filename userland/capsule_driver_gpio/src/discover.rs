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

//! The GPIO controller records the kernel published (ACPI bus, class
//! GPIO_CTRL), each claimed and every window it lists mapped at its own
//! length. A record another capsule holds, or one whose windows do not
//! map, is said and passed over; its claim is given back.

use crate::bank::Bank;
use crate::ctl::ctl;
use crate::irq::bind;
use crate::mmio::Mmio;
use crate::name::name;
use crate::say::line;
use alloc::format;
use alloc::vec::Vec;
use nonos_gpio::Router;
use nonos_libc::{
    mk_device_claim, mk_device_list, mk_device_release, mk_mmio_map, DeviceRecord, MmioMapOut,
    BUS_KIND_ACPI,
};

const CLASS_GPIO_CTRL: u32 = 0x0080;
const MAX_DEVICES: usize = 128;

fn windows(r: &DeviceRecord, epoch: u64) -> Option<Vec<Mmio>> {
    let mut out = Vec::new();
    for bar in 0..r.bar_count {
        let size = r.bars.get(usize::from(bar))?.size;
        let mut m = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
        if mk_mmio_map(r.device_id, epoch, bar.into(), 0, 0, size, &mut m) < 0 {
            return None;
        }
        out.push(Mmio { va: m.user_va, len: size.min(m.length) });
    }
    Some(out)
}

pub fn banks() -> Vec<Bank> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { (n as usize).min(MAX_DEVICES) } else { 0 };
    let ours = |r: &&DeviceRecord| r.bus_kind == BUS_KIND_ACPI && r.class == CLASS_GPIO_CTRL;
    let mut out = Vec::new();
    for r in buf[..count].iter().filter(ours) {
        let epoch = mk_device_claim(r.device_id);
        if epoch <= 0 {
            line(&format!("controller {} held elsewhere ({}), passed over", r.device, epoch));
            continue;
        }
        let hid = name(r.device_id).map_or([0; 8], |n| n.hid);
        let Some(c) = windows(r, epoch as u64).and_then(|w| ctl(hid, w)) else {
            line(&format!("controller {}: windows did not map, released", r.device));
            let _ = mk_device_release(r.device_id);
            continue;
        };
        let (level_aei, grant) = (Vec::new(), bind(r, epoch as u64));
        let router = Router::default();
        out.push(Bank { record: *r, uid: r.device, ctl: c, router, level_aei, grant, seq: 0 });
    }
    out
}
