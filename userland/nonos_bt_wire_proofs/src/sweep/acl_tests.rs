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
//! ACL packets (Vol 4, Part E, 5.4.2): a sound packet reads as built; a
//! lying length, a reserved handle or a broadcast flag is refused.

use super::rng::each;
use nonos_bt_wire::hci::{acl, parse_acl, Boundary};
use nonos_bt_wire::WireError;

#[test]
fn packets_read_as_built_and_lies_are_refused() {
    each(|rng| {
        let (handle, data) = (rng.below(0x0f00) as u16, rng.span(0, 64));
        let b = Boundary::from_bits(rng.below(4) as u16);
        let p = acl(handle, b, &data).unwrap();
        let (head, got) = parse_acl(&p).expect("sound");
        assert_eq!((head.handle, head.boundary, got), (handle, b, &data[..]));
        let mut lie = p.clone();
        let n = (data.len() as u16).wrapping_add(1 + rng.below(0xfffe) as u16);
        lie[2..4].copy_from_slice(&n.to_le_bytes());
        assert!(parse_acl(&lie).is_err(), "a stated length of {n}");
        let mut lie = p.clone();
        lie[1] |= if rng.below(2) == 0 { 0x40 << rng.below(2) } else { 0x0f };
        assert_eq!(parse_acl(&lie).err(), Some(WireError::Reserved));
    });
}
