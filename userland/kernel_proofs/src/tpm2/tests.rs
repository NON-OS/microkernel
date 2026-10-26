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

//! TPM2 tables as the TCG ACPI specification lays them out (recalled,
//! unverified): log area or not, bad lengths and sums, the start methods.

use super::tpm2_start::*;
use super::tpm2_table::{decode, Tpm2Fault, MAX_LEN};

fn seal(t: &mut [u8]) {
    let len = t.len() as u32;
    t[4..8].copy_from_slice(&len.to_le_bytes());
    t[9] = 0;
    t[9] = 0u8.wrapping_sub(t.iter().fold(0u8, |s, b| s.wrapping_add(*b)));
}

fn tpm2(method: u32, with_log: bool) -> Vec<u8> {
    let mut t = b"TPM2".to_vec();
    t.extend_from_slice(&[0; 32]);
    t.extend_from_slice(&[1, 0, 0, 0]);
    t.extend_from_slice(&0xFED4_0040u64.to_le_bytes());
    t.extend_from_slice(&method.to_le_bytes());
    t.extend_from_slice(&[0xA5; 12]);
    if with_log {
        t.extend_from_slice(&0x1_0000u32.to_le_bytes());
        t.extend_from_slice(&0x7F00_0000u64.to_le_bytes());
    }
    seal(&mut t);
    t
}

#[test]
fn with_and_without_the_log_area() {
    let t = decode(&tpm2(START_CRB, true)).unwrap();
    assert_eq!((t.platform_class, t.control_area, t.start_method), (1, 0xFED4_0040, 7));
    assert_eq!(t.parameters, [0xA5; 12]);
    assert_eq!(t.log_area, Some((0x1_0000, 0x7F00_0000)));
    let t = decode(&tpm2(START_FIFO, false)).unwrap();
    assert_eq!((t.start_method, t.parameters.len(), t.log_area), (6, 12, None));
}

#[test]
fn bad_lengths_and_checksums() {
    let mut t = tpm2(START_CRB, false);
    t[20] ^= 1;
    assert!(matches!(decode(&t), Err(Tpm2Fault::Checksum(_))));
    let short = tpm2(START_CRB, false);
    assert_eq!(decode(&short[..40]), Err(Tpm2Fault::Length(64)), "fewer bytes than it says");
    let mut long = tpm2(START_CRB, false);
    long[4..8].copy_from_slice(&(MAX_LEN as u32 + 1).to_le_bytes());
    assert_eq!(decode(&long), Err(Tpm2Fault::Length(MAX_LEN + 1)));
    assert_eq!(decode(&[0; 4]), Err(Tpm2Fault::Length(0)));
}
