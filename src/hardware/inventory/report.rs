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

//! The hardware report in the boot log: one `[HW]` line per device the
//! broker lists, saying which driver takes it, why none does, that none is
//! needed, or that no table knows it, then a count. A person reads it with
//! `log HW` in the Terminal and can send it as written (report_text.rs).

use super::record::InventoryRecord;
use super::report_extra::extra;
use super::report_text::{text, Ids, Verdict, TEXT_MAX};
use crate::hardware::broker::{list, BusKind, DeviceRecord};
use crate::sys::serial::Line;

pub fn say() {
    let mut counts = [0u64; 4];
    for rec in list() {
        let (ids, v) = (ids(&rec), verdict(&rec));
        counts[match v {
            Verdict::Driver(_) => 0,
            Verdict::Held(_) => 1,
            Verdict::NotNeeded => 2,
            Verdict::Unknown => 3,
        }] += 1;
        let mut buf = [0u8; TEXT_MAX];
        let n = text(&mut buf, &ids, &v);
        Line::new().str(&buf[..n]).end();
    }
    let mut l = Line::new();
    l.str(b"[HW] ").dec(counts[0]).str(b" driven, ").dec(counts[1]).str(b" known and held, ");
    l.dec(counts[2]).str(b" need none, ").dec(counts[3]).str(b" unknown").end();
}

fn ids(rec: &DeviceRecord) -> Ids {
    let pair = (rec.vendor, rec.device);
    match rec.bus_kind {
        b if b == BusKind::Pci as u8 => Ids::Pci(pair, (rec.pci_class, rec.pci_subclass)),
        b if b == BusKind::Acpi as u8 && rec.vendor == 1 => Ids::Pnp(rec.device),
        b if b == BusKind::Acpi as u8 => Ids::Other(b"acpi", pair),
        b if b == BusKind::Virt as u8 => Ids::Other(b"virt", pair),
        b if b == BusKind::Fdt as u8 => Ids::Other(b"fdt", pair),
        _ => Ids::Other(b"firmware", pair),
    }
}

fn verdict(rec: &DeviceRecord) -> Verdict<'static> {
    let r = InventoryRecord::from_device_record(rec);
    let pci = rec.bus_kind == BusKind::Pci as u8;
    let pnp = rec.bus_kind == BusKind::Acpi as u8 && rec.vendor == 1;
    if let Some(v) = extra(pci, pnp, (rec.vendor, rec.device), r.family) {
        return v;
    }
    match (r.driver, r.missing) {
        (Some(d), _) => Verdict::Driver(d),
        (None, Some(why)) => Verdict::Held(why),
        (None, None) => Verdict::Unknown,
    }
}
