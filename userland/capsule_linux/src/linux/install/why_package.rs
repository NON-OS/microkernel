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

//! Why an install stopped at a package: its index, its download, landing
//! it, or taking it off again (why.rs).

use super::Why;

impl Why {
    /// The index or database did not fetch, or its signature did not verify.
    pub const INDEX: Self = Self(2);
    /// A package, or something it depends on, is in no index.
    pub const NOT_PROVIDED: Self = Self(3);
    /// The closure passes the configured package limit.
    pub const TOO_LARGE: Self = Self(4);
    /// A package did not download, or did not match its pin, checksum or
    /// signature.
    pub const PACKAGE: Self = Self(5);
    /// The image was built without a mirror for this family.
    pub const NO_MIRROR: Self = Self(8);
    /// The image was built without a keyring for this family.
    pub const NO_KEYRING: Self = Self(9);
    /// A package did not land whole: a file the store would not take, or
    /// links the table could not hold. What was written of it is gone.
    pub const PARTIAL: Self = Self(21);
    /// No install of the package is recorded on this system.
    pub const NOT_INSTALLED: Self = Self(22);
    /// Not every file or link of it would go; what is left is still
    /// recorded, so asking again finishes the job.
    pub const REMOVE_PARTIAL: Self = Self(24);
    /// A shipped tier's program (the chat window, /bin/qwenchat) is not in
    /// this system's store, so the tier cannot be installed from it. Said
    /// apart from NotProvided, which reads as a package index that lacks
    /// something: the market's index was whole, and the store was not.
    pub const PROGRAM_MISSING: Self = Self(34);
    /// The store holds the program and it could not be read; the serial
    /// line before the exit names the store's reason.
    pub const PROGRAM_UNREADABLE: Self = Self(35);
    /// A market program's policy was not one this machine may vouch for: not
    /// canonical, not market ready, naming another listing, release or
    /// program, or not written. Nothing of the package is left (`market`).
    pub const POLICY: Self = Self(36);
    /// A package would not fit in free memory to unpack; nothing of it landed.
    pub const UNPACK_MEMORY: Self = Self(37);
}
