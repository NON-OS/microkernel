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

//! The exit status is part of the interface, as the verdict format is: a
//! script may act on it without reading the verdict. These values do not
//! change within verdict format 1.

/// Every check ran and passed, over a release.
pub const GO: u8 = 0;
/// At least one check failed or could not run.
pub const NO_GO: u8 = 1;
/// The command line was not understood; nothing was checked.
pub const USAGE: u8 = 2;
/// The checks ran, but the verdict could not be written or signed as asked.
pub const NOT_DELIVERED: u8 = 3;
/// `verify-signature`: the signature does not verify, or could not be checked.
pub const SIGNATURE_BAD: u8 = 4;
/// Every check ran and passed, over a development image: GO, but never a
/// release, so a script acting on the status alone cannot deploy it as one.
pub const DEV_GO: u8 = 5;
