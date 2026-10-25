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

//! Which functions' claims take the boot framebuffer's scanout (L79): found
//! once at boot, after every PCI record and the legacy records exist and
//! before the framebuffer's own record is published, then read on every claim
//! with no lock. Every record then present counts, of any bus; the firmware's
//! I2C and GPIO controllers, registered later, are not asked.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use spin::Once;

use super::cover_rule::{covers as rule, holds, Span};
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{BarKind, BusKind, DeviceRecord};
use crate::hardware::broker::table;
use crate::sys::serial;

static COVER: Once<Vec<u64>> = Once::new();

fn display(r: &DeviceRecord) -> bool {
    r.bus_kind == BusKind::Pci as u8 && r.class == ids::DISPLAY
}

/// From the framebuffer's first page and its length past that page.
pub(super) fn find(page: u64, len: u64) {
    let all = table::list();
    let mine = |r: &DeviceRecord| {
        let spans: [Span; 6] = r.bars.map(|b| (b.base, b.size, b.kind));
        holds((page, len), &spans, usize::from(r.bar_count), BarKind::Mmio as u8)
    };
    let any = all.iter().any(mine);
    let single = all.iter().filter(|r| display(r)).count() == 1;
    let by = |r: &&DeviceRecord| rule(mine(r), any, display(r), single);
    let ids: Vec<u64> = all.iter().filter(by).map(|r| r.device_id).collect();
    for id in &ids {
        let how = if any { "lies in its memory" } else { "lies in no BAR; it is the one display" };
        let line = format!(
            "[BROKER] boot framebuffer {}: device {}'s claim stops the console for good",
            how, id
        );
        serial::println(line.as_bytes());
    }
    if ids.is_empty() {
        serial::println(b"[BROKER] boot framebuffer: no device is known to drive its scanout");
    }
    COVER.call_once(|| ids);
}

/// A claim of `device_id` takes the scanout.
pub(in crate::hardware::broker) fn covers(device_id: u64) -> bool {
    COVER.get().is_some_and(|v| v.contains(&device_id))
}
