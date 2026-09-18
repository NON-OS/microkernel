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

//! What one step reports.

use crate::writer::Receipt;

pub enum Progress<'a> {
    /// Bytes of the volume written so far, out of the total the plan holds.
    Writing { done: u64, total: u64 },
    /// The volume is complete and the partition table just went down.
    TableWritten,
    /// Flushed. The receipt is the last word.
    Done(Receipt<'a>),
}
