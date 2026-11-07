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

//! Why an image, its chunk list or a chunk is refused. Each is whole:
//! nothing of a refused object is used.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// Not the magic, or a version, flags or reserved bytes this parser
    /// does not read.
    Magic,
    /// Shorter than its header says, or its lengths do not add up to the
    /// object's length.
    Layout,
    /// Past MAX_IMAGE, MAX_MEMBERS or MAX_RAW, or a chunk size out of range.
    TooLarge,
    /// A member name that is not a package name, or one named twice.
    Name,
    /// A codec or role this parser does not read, the listed member not
    /// first, or a second listed member.
    Member,
    /// Bytes that are not the digest their table or list gives them.
    Digest,
    /// The builder was handed something the parser would not take back.
    Build,
}
