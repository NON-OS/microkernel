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

//! The SAE exchange's own copies of its scalar and element and of what
//! the peer sent, wiped when the station is dropped. The commit, with the
//! secret rand and password element, and the keys wipe themselves.

use super::*;
use crate::vault::{assert_not_copy, wipe};

assert_not_copy!(SaeStation);

impl Drop for SaeStation {
    fn drop(&mut self) {
        wipe(&mut self.own_scalar);
        wipe(&mut self.own_element);
        wipe(&mut self.peer_scalar);
        wipe(&mut self.peer_element);
    }
}
