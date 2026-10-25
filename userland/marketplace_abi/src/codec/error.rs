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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    Short,
    TooLarge,
    TooManyItems,
    BadUtf8,
    UnsupportedSchema,
    BlobTooLarge,
    /// A price kind, settle mode or proof kind this build does not know.
    UnknownKind,
    /// A guest policy that is neither empty nor exactly 256 bytes.
    BadLength,
    /// Bytes after the index signature, which nothing signed.
    Trailing,
    /// An escrow listing whose release has no chunk root or length.
    Inconsistent,
}
