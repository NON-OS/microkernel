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

//! A table that could name this one unit never goes with its digest.

use nonos_hwreport::acpi::{hashed, line, RECORD_LEN};
use nonos_hwreport::sink::Sink;

fn record(sig: &[u8; 4]) -> [u8; RECORD_LEN] {
    let mut r = [0xabu8; RECORD_LEN];
    r[..4].copy_from_slice(sig);
    r[4..8].copy_from_slice(&1u32.to_be_bytes());
    r[8..12].copy_from_slice(&4096u32.to_be_bytes());
    r
}

fn text(rec: &[u8]) -> Option<String> {
    let mut buf = [0u8; 200];
    let mut out = Sink::new(&mut buf);
    line(rec, &mut out).then(|| String::from_utf8(out.written().to_vec()).unwrap())
}

#[test]
fn model_tables_carry_their_digest() {
    let got = text(&record(b"DSDT")).unwrap();
    assert_eq!(got, format!("acpi DSDT 1 4096 {}\n", "ab".repeat(32)));
    assert!(hashed(b"SSDT") && hashed(b"DMAR") && hashed(b"IVRS"));
}

#[test]
fn product_key_and_unknown_tables_keep_only_their_name_and_length() {
    for sig in [b"MSDM", b"SLIC", b"BGRT", b"FPDT", b"BERT", b"OEM1", b"WPBT"] {
        assert!(!hashed(sig));
        let got = text(&record(sig)).unwrap();
        assert!(got.ends_with(" 1 4096 -\n"), "{got}");
        assert!(!got.contains("abab"), "{got}");
    }
}

#[test]
fn a_record_of_the_wrong_size_or_a_strange_name_is_handled() {
    assert_eq!(text(&[0u8; RECORD_LEN - 1]), None);
    assert_eq!(text(&[0u8; RECORD_LEN + 1]), None);
    let got = text(&record(b"a\n\x00Z")).unwrap();
    assert!(got.starts_with("acpi ___Z 1 4096 -"), "{got}");
}
