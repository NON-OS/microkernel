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

/// Why the encoder refused an index: it writes only what the decoder reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeError {
    /// An entry with no commerce block, which schema 3 requires.
    MissingCommerce,
    /// A release with no schema 3 fields.
    MissingReleaseExt,
    /// A guest policy that is neither empty nor exactly 256 bytes.
    BadLength,
    /// More entries, releases, proof requirements or anchors than the caps.
    TooMany,
    /// A proof requirement of a kind the decoder would refuse.
    UnknownKind,
    /// An escrow listing whose release has no chunk root or length.
    Inconsistent,
}
