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

//! How long the device may take, by the clock.

/// How long one request may stay with the device before it is taken to have
/// stopped. QEMU answers at once; a rate-limited backend takes longer.
pub const FILL_MS: u64 = 500;

/// How long a device may take to read back zero after a reset.
pub const RESET_MS: u64 = 1_000;
