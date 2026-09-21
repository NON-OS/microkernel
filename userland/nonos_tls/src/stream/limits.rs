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

//! Record and buffer bounds, from RFC 8446 section 5.2.

/// Largest protected record body: 2^14 plus 256 for the expansion.
pub(super) const BODY_MAX: usize = (1 << 14) + 256;

pub(super) const PLAINTEXT_MAX: usize = 1 << 14;

pub(super) const PARTIAL_MAX: usize = 256 * 1024;

pub(super) const FLIGHT_MAX: usize = 128 * 1024;
