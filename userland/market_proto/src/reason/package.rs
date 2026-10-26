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

//! A package, its index or this system's build stopped it.

use super::{said, Reason};

/// The index or database did not fetch, or its signature did not verify.
pub const WHY_INDEX: u8 = 2;
/// A package, or something it depends on, is in no index.
pub const WHY_NOT_PROVIDED: u8 = 3;
/// The closure passes the configured package limit.
pub const WHY_TOO_LARGE: u8 = 4;
/// A package did not download, or did not match its pin, checksum or
/// signature.
pub const WHY_PACKAGE: u8 = 5;
/// The image was built without a mirror for this family.
pub const WHY_NO_MIRROR: u8 = 8;
/// The image was built without a keyring for this family.
pub const WHY_NO_KEYRING: u8 = 9;
/// Not every file of a package could be written into the store; what was
/// written of it was taken out again.
pub const WHY_PARTIAL: u8 = 21;
/// The install role could not check or vouch for a market program's policy
/// (plan/30). Nothing produces it before I-C5.
pub const WHY_POLICY: u8 = 36;
/// A package would not fit in free memory to unpack; nothing of it landed.
pub const WHY_UNPACK_MEMORY: u8 = 37;

const POLICY_LINE: &str =
    "The market's policy for this program could not be checked or vouched for; nothing was installed.";

const UNPACK_LINE: &str =
    "Too little free memory to unpack one of its packages; close other apps, then retry";

pub(super) fn package_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_INDEX => said("The package index did not download or verify", true),
        WHY_NOT_PROVIDED => said("Something it needs is in no index", true),
        WHY_TOO_LARGE => said("It needs more packages than this machine allows", false),
        WHY_PACKAGE => said("A package did not download, or did not verify", true),
        WHY_NO_MIRROR => said("This system has no mirror for it", false),
        WHY_NO_KEYRING => said("This system holds no key to check it with", false),
        WHY_PARTIAL => said("Not every file could be written, so none of it was kept", true),
        WHY_POLICY => said(POLICY_LINE, false),
        WHY_UNPACK_MEMORY => said(UNPACK_LINE, true),
        _ => return None,
    })
}
