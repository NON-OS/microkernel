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

//! The header held to its layout: a pack whose header is not exactly the
//! layout's is refused whole, with the reason.

mod common;
#[path = "common/edit.rs"]
mod edit;

use edit::{put, refused};
use nonos_linux_pack::{Pack, Refused, MAX_ENTRIES};

#[test]
fn the_header_is_exactly_the_layout() {
    assert_eq!(refused(|b| b[0] = b'X'), Refused::Magic);
    assert_eq!(refused(|b| put(b, 8, &2u16.to_le_bytes())), Refused::Magic);
    assert_eq!(refused(|b| put(b, 10, &1u16.to_le_bytes())), Refused::Magic);
    assert_eq!(refused(|b| put(b, 16, &65u64.to_le_bytes())), Refused::Layout);
    assert_eq!(refused(|b| put(b, 48, &0u64.to_le_bytes())), Refused::Layout);
    assert_eq!(refused(|b| b[56] = 1), Refused::Layout);
    assert_eq!(refused(|b| b.push(0)), Refused::Layout, "a byte past the last file");
    assert_eq!(refused(|b| b.truncate(b.len() - 1)), Refused::Data);
    let many = (MAX_ENTRIES as u32 + 1).to_le_bytes();
    assert_eq!(refused(|b| put(b, 12, &many)), Refused::TooLarge);
    assert_eq!(Pack::parse(&[]).err(), Some(Refused::Magic));
}
