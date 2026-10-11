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

use super::sample::policy;
use crate::{decode, PolicyError, LEN};

#[test]
fn refuses_wrong_magic_and_version() {
    let mut b = policy().encode();
    b[7] = b'2';
    assert_eq!(decode(&b), Err(PolicyError::Magic));
    let mut b = policy().encode();
    b[8] = 2;
    assert_eq!(decode(&b), Err(PolicyError::Version));
    let mut b = policy().encode();
    b[9] = 1;
    assert_eq!(decode(&b), Err(PolicyError::Version));
}

#[test]
fn refuses_every_reserved_byte() {
    for at in (10..16).chain(217..LEN) {
        let mut b = policy().encode();
        b[at] = 0x80;
        assert_eq!(decode(&b), Err(PolicyError::Reserved), "byte {at}");
    }
}

#[test]
fn refuses_unnamed_mask_bits_and_calls_past_the_last_bucket() {
    for bit in 9..64 {
        let mut b = policy().encode();
        b[48..56].copy_from_slice(&(1u64 << bit).to_le_bytes());
        assert_eq!(decode(&b), Err(PolicyError::UnknownMask), "bit {bit}");
    }
    for bit in 1..8 {
        let mut b = policy().encode();
        b[216] |= 1 << bit;
        assert_eq!(decode(&b), Err(PolicyError::SyscallSet), "bit {bit}");
    }
}
