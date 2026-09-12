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

//! The capability bits an invariant actually probes.
//!
//! Named rather than shifted inline so a check and the capsule manifest that
//! grants the bit can be read against each other. Only probed bits are listed: a
//! constant here that nothing checks is a check someone believed existed.

pub const CAP_DEBUG: u64 = 1 << 8;
pub const CAP_ADMIN: u64 = 1 << 9;
