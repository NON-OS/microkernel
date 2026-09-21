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

//! The chain check against a real relay.

use crate::link::bind;
use crate::vectors::{CAPTURED_AT, CERTS, IDENTITY, LEAF};

#[test]
fn a_real_relay_binds() {
    assert_eq!(bind(CERTS, LEAF, &IDENTITY, CAPTURED_AT), Ok(()));
}
