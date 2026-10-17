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

//! The capsule's pinned check with one pinned form changed, or the top's
//! root touched: it fails, and the FAIL lines name the vector and the form.

use super::forms::{check, pinned_as};
use super::json::{failed, parts};
use super::top::{flipped, SYNTHETIC};
use crate::vectors::ALL;

#[test]
fn one_pinned_line_changed_fails_by_name() {
    for v in &ALL {
        let mut json = v.proof_json.to_vec();
        let at = json.windows(12).position(|w| w == b"\"publics\": [").expect("publics") + 12;
        json[at] = if json[at] == b'1' { b'2' } else { b'1' };
        let (ok, text) = check(v, &pinned_as(v, json, v.format7.to_vec()), SYNTHETIC);
        let n = parts(v).0.len();
        assert!(!ok);
        assert_eq!(
            failed(&text),
            [format!("{}: proof.json byte for byte ({n} proof bytes)", v.name)]
        );
        let mut format7 = v.format7.to_vec();
        *format7.last_mut().expect("format 7 bytes") ^= 1;
        let (ok, text) = check(v, &pinned_as(v, v.proof_json.to_vec(), format7), SYNTHETIC);
        let name =
            format!("{}: proof-format7.bin byte for byte ({} bytes)", v.name, v.format7.len());
        assert!(!ok);
        assert_eq!(failed(&text), [name]);
    }
}

#[test]
fn a_top_with_one_root_bit_flipped_fails_by_name() {
    for v in &ALL {
        let (ok, text) = check(v, v, &flipped(0));
        assert!(!ok);
        let format7 =
            format!("{}: proof-format7.bin byte for byte ({} bytes)", v.name, v.format7.len());
        assert_eq!(failed(&text), [format7, format!("{}: the format 7 form verifies", v.name)]);
    }
}
