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

//! F-105b: the kernel's entry-point reader held to the library's on the
//! F-105 seeds. Firmware's memory holds the entry point, honest or lying,
//! then anything. The kernel copies the entry point's own length, then the
//! range `table_span` names; whenever it copies a table, the library
//! accepts the copy and names the same table, and reads no byte past it.

use super::seeded_firmware::firmware;
use super::seeded_gen::Rng;
use crate::entry::parse_entry as library;
use crate::kernel::smbios_copy::{table_span, LEN_AT};
use crate::kernel::smbios_entry::{entry_len, parse_entry as kernel, ENTRY_MAX, TABLE_MAX};

#[test]
fn the_kernel_copies_only_what_the_library_reads() {
    let (mut wide_copies, mut narrow_copies, mut stricter, mut wraps) = (0, 0, 0, 0);
    for seed in 1..=6000u64 {
        let r = &mut Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let (wide, memory) = firmware(r);
        let Some(n) = entry_len(&memory[..LEN_AT]) else { continue };
        assert!(n <= ENTRY_MAX, "seed {seed}");
        let entry = &memory[..n];
        let Some((start, end)) = table_span(entry) else {
            let lib = library(entry);
            stricter += usize::from(lib.is_some());
            wraps += usize::from(lib.is_some_and(|e| e.table.checked_add(e.len.into()).is_none()));
            continue;
        };
        let lib = library(entry).unwrap_or_else(|| panic!("seed {seed}: {entry:x?}"));
        let k = kernel(entry).unwrap();
        assert_eq!((lib.table, u64::from(lib.len)), (start, end - start), "seed {seed}");
        assert_eq!((lib.major, lib.minor), (k.major, k.minor), "seed {seed}");
        assert!(start < end && end - start <= u64::from(TABLE_MAX), "seed {seed}");
        // The library reads to the copy's own length byte: the copy's end.
        assert_eq!(usize::from(entry[5 + usize::from(wide)]), n, "seed {seed}");
        *(if wide { &mut wide_copies } else { &mut narrow_copies }) += 1;
    }
    let floors = wide_copies >= 300 && narrow_copies >= 300 && stricter >= 100 && wraps >= 50;
    let says = format!("{wide_copies} wide, {narrow_copies} narrow, {stricter} stricter");
    assert!(floors, "copies {says}, {wraps} wrapping");
}
