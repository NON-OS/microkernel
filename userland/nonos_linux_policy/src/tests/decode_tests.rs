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
fn round_trips() {
    let p = policy();
    assert_eq!(decode(&p.encode()), Ok(p));
}

#[test]
fn fields_sit_where_the_plan_puts_them() {
    let b = policy().encode();
    assert_eq!(&b[0..8], b"NLXPOL01");
    assert_eq!(&b[8..10], &[1, 0]);
    assert_eq!(&b[16..48], &[0x11; 32]);
    assert_eq!(&b[48..56], &0x21u64.to_le_bytes());
    assert_eq!(&b[56..88], &[0x22; 32]);
    assert_eq!(&b[88..120], &[0x33; 32]);
    assert_eq!(&b[120..152], &[0x44; 32]);
    // calls 0 and 1, 231, 511, and 9999 in the shared last bucket
    assert_eq!(b[152], 0b11);
    assert_eq!(b[152 + 28], 1 << 7);
    assert_eq!(b[152 + 63], 1 << 7);
    assert_eq!(b[152 + 64], 1);
    assert!(b[217..].iter().all(|&x| x == 0));
}

#[test]
fn refuses_any_other_length() {
    let b = policy().encode();
    assert_eq!(decode(&b[..LEN - 1]), Err(PolicyError::Length));
    let mut long = [0u8; LEN + 1];
    long[..LEN].copy_from_slice(&b);
    assert_eq!(decode(&long), Err(PolicyError::Length));
    assert_eq!(decode(&[]), Err(PolicyError::Length));
}
