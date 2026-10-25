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

//! An encoder's length field is one its peer's reader takes, and a body
//! one byte too long for the peer is refused before it is sent.

use super::stages::ALL;
use nsh_wire::{len_field, wire_len, WireError};

#[test]
fn len_field_agrees_with_wire_len() {
    for e in ALL {
        let field = len_field(e.cap() - 1, e).expect("largest body");
        assert!(wire_len(field, e).is_ok(), "{e:?}");
        assert_eq!(len_field(e.cap(), e), Err(WireError::Oversize), "{e:?}");
    }
}
