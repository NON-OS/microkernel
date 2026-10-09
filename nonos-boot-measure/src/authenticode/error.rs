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

/// Why an image has no Authenticode digest. Each has a stable code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeError {
    /// A header field runs past the end of the file.
    Truncated,
    /// No `MZ` and `PE\0\0` signatures where the format puts them.
    NotPe,
    /// An optional header magic other than PE32 or PE32+, or one too short
    /// for its own fields.
    BadOptionalHeader,
    /// More than `MAX_SECTIONS` sections.
    TooManySections,
    /// A section's raw data, the headers or the certificate table outside the
    /// file, or the certificate table overlapping what is hashed.
    OutOfFile,
}

impl PeError {
    pub const fn code(self) -> u32 {
        match self {
            PeError::Truncated => 1,
            PeError::NotPe => 2,
            PeError::BadOptionalHeader => 3,
            PeError::TooManySections => 4,
            PeError::OutOfFile => 5,
        }
    }
}
