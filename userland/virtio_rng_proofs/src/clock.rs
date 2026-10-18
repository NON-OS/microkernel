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

//! The clock the driver's waits are measured by, as the tests run it: each
//! question a wait asks counts as the millisecond the driver sleeps between
//! them.

pub fn expired_after(ms: u64) -> impl FnMut() -> bool {
    let mut spent = 0u64;
    move || {
        spent += 1;
        spent >= ms
    }
}
