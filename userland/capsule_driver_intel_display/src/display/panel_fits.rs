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

//! The sizes a primary plane can scan out (`plane::program`'s limits:
//! 8192 wide, 4096 high), the bound on what the monitor's modes may offer.
//! Whether a mode's clock fits CDCLK is mode setting's check, at the set.

pub fn fits(w: u32, h: u32) -> bool {
    (640..=8192).contains(&w) && (480..=4096).contains(&h)
}
