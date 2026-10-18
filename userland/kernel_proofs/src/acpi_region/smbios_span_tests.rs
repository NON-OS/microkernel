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

//! F-105b, the kernel's side: from an SMBIOS entry point to the bytes the
//! kernel copies. `table_span` names a range never empty, never wrapping
//! and never past `TABLE_MAX`; the table-read rule then copies it only when
//! every byte outside the legacy BIOS window is firmware's memory by the
//! map (the model below walks the map itself, apart from the rule).

#[path = "smbios_span_gen.rs"]
mod gen;
#[path = "smbios_span_model.rs"]
mod model;

use super::rule::table_read;
use crate::smbios::smbios_copy::table_span;
use crate::smbios::smbios_entry::{parse_entry, TABLE_MAX};
use gen::{entry, Rng};
use model::{firmware_only, laptop};

#[test]
fn only_firmware_memory_within_the_bound_is_copied() {
    let m = laptop();
    let (mut copied, mut ruled_out, mut wraps) = (0, 0, 0);
    for seed in 1..=8000u64 {
        let (b, _) = entry(&mut Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1));
        let Some((start, end)) = table_span(&b) else {
            let e = parse_entry(&b);
            wraps += usize::from(e.is_some_and(|e| e.table.checked_add(e.len.into()).is_none()));
            continue;
        };
        assert!(start < end && end - start <= u64::from(TABLE_MAX), "seed {seed}");
        let read = table_read(start, end - start, &m).is_ok();
        assert_eq!(read, firmware_only(&m, start, end), "seed {seed}: {start:#x}..{end:#x}");
        *(if read { &mut copied } else { &mut ruled_out }) += 1;
    }
    let says = format!("{copied} copied, {ruled_out} refused by the rule, {wraps} wrapping");
    assert!(copied >= 300 && ruled_out >= 300 && wraps >= 50, "{says}");
}
