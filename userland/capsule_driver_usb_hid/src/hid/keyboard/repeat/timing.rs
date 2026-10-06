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

//! The repeat delay, rate and the bound on one repeat.

/// Before the first repeat.
pub const DELAY_MS: u64 = 500;
/// Between repeats, about 30 a second.
pub const RATE_MS: u64 = 33;
/// A repeat this long stops on its own. A boot keyboard has no idle report
/// to say a key is still down, and one pulled out while a key is held sends
/// no release: without the bound that key would type until the next report.
pub const LIMIT_MS: u64 = 30_000;
