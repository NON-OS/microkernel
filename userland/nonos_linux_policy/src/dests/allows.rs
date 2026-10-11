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

//! Whether a list lets a guest reach one address or name on one port.

use super::entry::Kind;
use super::list::entries;

/// Where a guest asks to connect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target<'a> {
    V4([u8; 4]),
    V6([u8; 16]),
    Name(&'a [u8]),
}

/// Whether `list`, which `check` has accepted, lets a guest reach `to` on
/// `port`. An IPv4 address written as IPv6 (::ffff:a.b.c.d) is matched as
/// IPv4, a name without regard to case or one trailing dot, and nothing
/// matches across kinds.
pub fn allows(list: &[u8], to: Target<'_>, port: u16) -> bool {
    let to = plain(to);
    entries(list).any(|d| {
        let within =
            |addr: &[u8]| (0..usize::from(d.prefix)).all(|b| bit(d.body, b) == bit(addr, b));
        (d.first_port..=d.last_port).contains(&port)
            && match (d.kind, to) {
                (Kind::V4, Target::V4(a)) => within(&a),
                (Kind::V6, Target::V6(a)) => within(&a),
                (Kind::Name, Target::Name(n)) => d.body.eq_ignore_ascii_case(n),
                _ => false,
            }
    })
}

fn plain(to: Target<'_>) -> Target<'_> {
    match to {
        Target::V6([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d]) => {
            Target::V4([a, b, c, d])
        }
        Target::Name(n) => Target::Name(n.strip_suffix(b".").unwrap_or(n)),
        other => other,
    }
}

/// Bit `at` of `bytes`, counting from the most significant bit of byte 0.
pub(super) fn bit(bytes: &[u8], at: usize) -> bool {
    bytes.get(at / 8).is_some_and(|b| b & (0x80 >> (at % 8)) != 0)
}
