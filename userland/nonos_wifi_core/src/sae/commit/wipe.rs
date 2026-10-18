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

//! A commit's secret rand and password element, and the keys an exchange
//! yields, overwritten once they are done with.

use super::*;
use crate::vault::{assert_not_copy, wipe, wipe_value};

assert_not_copy!(Commit);
assert_not_copy!(SaeKeys);

impl Drop for Commit {
    fn drop(&mut self) {
        wipe_value(&mut self.rand, Scalar::ZERO);
        wipe_value(&mut self.scalar, Scalar::ZERO);
        wipe_value(&mut self.pwe, ProjectivePoint::IDENTITY);
        wipe_value(&mut self.element, ProjectivePoint::IDENTITY);
    }
}

impl Drop for SaeKeys {
    fn drop(&mut self) {
        wipe(&mut self.kck);
        wipe(&mut self.pmk);
        wipe(&mut self.pmkid);
    }
}
