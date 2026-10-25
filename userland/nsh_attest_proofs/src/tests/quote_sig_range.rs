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

//! Step 3, r and s in range: zero or the group order is refused.

use nsh_attest::Refusal;

use super::refused;
use crate::build::sig::tpmt;
use crate::build::Spec;

const N: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xBC, 0xE6, 0xFA, 0xAD, 0xA7, 0x17, 0x9E, 0x84, 0xF3, 0xB9, 0xCA, 0xC2, 0xFC, 0x63, 0x25, 0x51,
];

fn sig(f: fn([u8; 32], [u8; 32]) -> Vec<u8>) -> Refusal {
    refused(Spec { sig: f, ..Spec::default() })
}

#[test]
fn r_or_s_out_of_range_is_refused() {
    let s = Refusal::Signature;
    assert_eq!(sig(|_, s| tpmt(0x18, 0x0B, &[0; 32], &s)), s);
    assert_eq!(sig(|r, _| tpmt(0x18, 0x0B, &r, &[0; 32])), s);
    assert_eq!(sig(|_, s| tpmt(0x18, 0x0B, &N, &s)), s);
    assert_eq!(sig(|r, _| tpmt(0x18, 0x0B, &r, &N)), s);
}
