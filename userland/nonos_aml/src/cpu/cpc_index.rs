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

//! Where each _CPC entry sits (ACPI 6.5, 8.4.6.1).

/// The entries after NumEntries and Revision, in the package's order, so
/// `fields[HIGHEST]` is element 2. Revision 3 has all 21.
pub const FIELDS: usize = 21;
pub const HIGHEST: usize = 0;
pub const NOMINAL: usize = 1;
pub const LOWEST_NONLINEAR: usize = 2;
pub const LOWEST: usize = 3;
pub const GUARANTEED: usize = 4;
pub const DESIRED: usize = 5;
pub const MINIMUM: usize = 6;
pub const MAXIMUM: usize = 7;
pub const TOLERANCE: usize = 8;
pub const TIME_WINDOW: usize = 9;
pub const WRAPAROUND: usize = 10;
pub const REFERENCE_COUNTER: usize = 11;
pub const DELIVERED_COUNTER: usize = 12;
pub const LIMITED: usize = 13;
pub const ENABLE: usize = 14;
pub const AUTONOMOUS: usize = 15;
pub const ACTIVITY_WINDOW: usize = 16;
pub const ENERGY_PREFERENCE: usize = 17;
pub const REFERENCE_PERFORMANCE: usize = 18;
pub const LOWEST_MHZ: usize = 19;
pub const NOMINAL_MHZ: usize = 20;
