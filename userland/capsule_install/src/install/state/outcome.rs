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

//! What the install ended with.

use alloc::string::String;

/// What the install ended with: the receipt's identifiers on success, the
/// error and where it happened otherwise.
pub struct Outcome {
    pub disk_guid: [u8; 36],
    pub partition_guid: [u8; 36],
    pub bytes_written: u64,
    pub bytes_verified: u64,
    pub seconds: u64,
    pub error: Option<String>,
}
