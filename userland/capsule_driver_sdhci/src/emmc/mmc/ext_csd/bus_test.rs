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

//! The bus width test: the read-only fields read back the same.

use super::offsets::BUS_TEST_BYTES;
use super::register::ExtCsd;

impl ExtCsd {
    /// The same read-only fields as `other`: the transfer at the new bus
    /// width moved the register intact.
    pub fn bus_test_same(&self, other: &ExtCsd) -> bool {
        BUS_TEST_BYTES.iter().all(|&i| self.raw[i] == other.raw[i])
    }
}
