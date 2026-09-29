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

//! Why an install stopped, as the installer's exit code, so the system and
//! the store can say more than that it failed. The log line before the exit
//! says which package and which check.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Why {
    /// The index or database did not fetch, or its signature did not verify.
    Index = 2,
    /// A package, or something it depends on, is in no index.
    NotProvided = 3,
    /// The closure passes the configured package limit.
    TooLarge = 4,
    /// A package did not download, or did not match its pin, checksum or
    /// signature.
    Package = 5,
    /// The image was built without a mirror for this family.
    NoMirror = 8,
    /// The image was built without a keyring for this family.
    NoKeyring = 9,
}

impl Why {
    pub fn code(self) -> i32 {
        self as i32
    }
}
