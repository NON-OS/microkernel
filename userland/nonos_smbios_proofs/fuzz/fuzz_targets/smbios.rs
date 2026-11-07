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

//! Any bytes as firmware's SMBIOS entry point and the table after it: the
//! readers never panic; an entry point either reader takes sums to zero over
//! its own length; the kernel copies no more than `ENTRY_MAX` bytes of it and
//! a table span that is never empty, never over `TABLE_MAX` and never wraps,
//! the same table the library then names; every structure the walk returns,
//! and each of its strings, lies inside the table; and every text shown is
//! printable ASCII no longer than `MAX_TEXT`.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_smbios_proofs::entry::parse_entry as library;
use nonos_smbios_proofs::identity::identity;
use nonos_smbios_proofs::kernel::smbios_copy::{table_span, LEN_AT};
use nonos_smbios_proofs::kernel::smbios_entry::{entry_len, parse_entry as kernel, ENTRY_MAX, TABLE_MAX};
use nonos_smbios_proofs::quirk::{lookup, Device, DeviceMatch, DmiMatch, Field, Match, Quirk, QUIRKS};
use nonos_smbios_proofs::text::{text, MAX_TEXT};
use nonos_smbios_proofs::walk::{structures, END_OF_TABLE, MAX_STRUCTURES};

const PROBE: &[Quirk] = &[Quirk {
    name: "probe",
    reason: "fuzz",
    dmi: &[DmiMatch { field: Field::Vendor, is: Match::Prefix("A") }],
    device: DeviceMatch::Acpi { hid: "PNP0C09" },
    value: 1,
}];

fn sums_to_zero(b: &[u8]) -> bool {
    b.iter().fold(0u8, |s, x| s.wrapping_add(*x)) == 0
}

fn within(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    inner.is_empty() || (i >= o && i + inner.len() <= o + outer.len())
}

fn shown(s: &str) {
    assert!(s.len() <= MAX_TEXT && s.bytes().all(|b| (0x20..0x7F).contains(&b)), "{s:?}");
}

fn entry(data: &[u8]) -> Option<u16> {
    let lib = library(data);
    if let Some(e) = lib {
        let len = usize::from(data[5 + usize::from(data[3] == b'3')]);
        assert!(len <= data.len() && sums_to_zero(&data[..len]), "{e:?}");
    }
    if let Some(k) = kernel(data) {
        assert!(k.len > 0 && k.len <= TABLE_MAX, "{k:?}");
        let e = lib.expect("the library takes what the kernel takes");
        assert_eq!((e.table, e.len, e.major, e.minor), (k.table, k.len, k.major, k.minor));
    }
    let n = entry_len(data.get(..LEN_AT)?)?;
    assert!(n <= ENTRY_MAX);
    let copied = data.get(..n)?;
    if let Some((start, end)) = table_span(copied) {
        assert!(start < end && end - start <= u64::from(TABLE_MAX));
        let lib = library(copied).expect("the library takes what the kernel copies");
        assert_eq!((lib.table, u64::from(lib.len)), (start, end - start));
    }
    lib.and_then(|e| e.count)
}

fn table(t: &[u8], count: Option<u16>) {
    let all = structures(t, count);
    let limit = count.map_or(MAX_STRUCTURES, |c| usize::from(c).min(MAX_STRUCTURES));
    assert!(all.len() <= limit);
    for (i, s) in all.iter().enumerate() {
        assert!(s.formatted.len() >= 4 && within(t, s.formatted));
        assert!(s.strings.iter().all(|x| within(t, x) && !x.contains(&0)));
        assert!(s.kind != END_OF_TABLE || i + 1 == all.len());
        for off in 0..s.formatted.len() {
            if s.byte(off) == Some(0) {
                assert!(s.string(off).is_none());
            }
            if let Some(x) = s.string(off) {
                shown(&text(x));
            }
        }
    }
    let id = identity(&all);
    for f in [&id.vendor, &id.product, &id.version, &id.board_vendor, &id.board] {
        shown(f);
    }
    for f in [&id.bios_vendor, &id.bios_version, &id.bios_date] {
        shown(f);
    }
    assert!(!id.chassis.is_empty());
    let hid = t.get(..t.len().min(8)).unwrap_or_default();
    for dev in [Device::Acpi { hid }, Device::Pci { vendor: 0x8086, device: 0x1234 }] {
        assert!(lookup(QUIRKS, &id, &dev).is_empty());
        let hits = lookup(PROBE, &id, &dev);
        assert!(hits.is_empty() || (id.vendor.starts_with('A') && hid == b"PNP0C09"));
    }
}

fuzz_target!(|data: &[u8]| {
    let count = entry(data);
    let split = data.first().map_or(0, |b| usize::from(*b & 0x1F)).min(data.len());
    table(&data[split..], count);
    table(data, None);
});
