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
//! HTTP/1.1 for a client, with no I/O of its own.

//! How large a download may be.

/// 200 MB: a long album side at a high bitrate, and well inside what the
/// session's RAM files can hold beside everything else.
pub const MAX_BYTES: u64 = 200 * 1024 * 1024;

/// What a person is told when a file is larger.
pub const TOO_LARGE: &str = "That file is larger than 200 MB, the most Music downloads.";
