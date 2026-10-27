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
//! broker lists, saying which driver takes it, why none does, or that no
//! table knows it, then a count. A person reads it with `log HW` in the
//! Terminal and can send it as written (report_text.rs says why).

use super::record::InventoryRecord;
use super::report_text::{text, Verdict, TEXT_MAX};
use super::scan::scan;
use crate::sys::serial::Line;

pub fn say() {
    let (mut driven, mut held, mut unknown) = (0u64, 0u64, 0u64);
    for r in scan() {
        let v = verdict(&r);
        match v {
            Verdict::Driver(_) => driven += 1,
            Verdict::Held(_) => held += 1,
            Verdict::Unknown => unknown += 1,
        }
        let mut buf = [0u8; TEXT_MAX];
        let n = text(&mut buf, (r.vendor, r.device), (r.pci_class, r.pci_subclass), &v);
        Line::new().str(&buf[..n]).end();
    }
    let mut l = Line::new();
    l.str(b"[HW] ").dec(driven).str(b" driven, ").dec(held).str(b" known and held, ");
    l.dec(unknown).str(b" unknown").end();
}

fn verdict(r: &InventoryRecord) -> Verdict<'static> {
    match (r.driver, r.missing) {
        (Some(d), _) => Verdict::Driver(d),
        (None, Some(why)) => Verdict::Held(why),
        (None, None) => Verdict::Unknown,
    }
}
