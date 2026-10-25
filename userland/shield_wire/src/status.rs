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

//! The statuses a reply carries.

/// Done, and the values are the answer.
pub const STATUS_OK: i32 = 0;
/// A job started; ask for RESULT.
pub const STATUS_STARTED: i32 = 1;
/// The worker is busy with another job.
pub const STATUS_BUSY: i32 = 2;
/// Refused, with `why` naming the reason in one sentence.
pub const STATUS_REFUSED: i32 = 3;
/// Not a request this service reads.
pub const STATUS_MALFORMED: i32 = -22;
/// Not from the wallet.
pub const STATUS_DENIED: i32 = -13;
