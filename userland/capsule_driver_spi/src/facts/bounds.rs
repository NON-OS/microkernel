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
//! Every bound a wait has, this driver's own: the core is polled, and a
//! frame at the slowest clock it programs takes microseconds.

/// A whole transfer, at most TRANSFER_MAX bytes.
pub const TRANSFER_MS: u64 = 100;
/// The most bytes one request moves, both ways at once.
pub const TRANSFER_MAX: usize = 4000;
