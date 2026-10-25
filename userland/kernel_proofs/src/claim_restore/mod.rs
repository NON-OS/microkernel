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

/*
 * What a claimed function's BARs must read once it is in D0 (L62), the
 * kernel's rule included by path: every kind's address bits, the type bits
 * left as the device has them, and a 64-bit BAR's high half its own slot.
 */

#[path = "../../../../src/hardware/broker/claim/restore_rule.rs"]
mod restore_rule;
mod tests;
