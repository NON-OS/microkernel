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

//! A pack's trailer, minted over its index and then asked of the kernel as
//! a run asks it (file/store/pack/load.rs). A machine that setup left at
//! "Only NONOS software" mints a trailer it then refuses to serve under, so
//! the install stops at its first package and says why, rather than
//! fetching every package for a program that would never start.

use alloc::vec::Vec;

/// Why a pack is not landed with a trailer a run would take.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Unadmitted {
    /// No trailer: no enrolled identity, or a capsule that may not mint.
    NoMint,
    /// A trailer this machine would not load the pack under.
    NotRun,
}

impl Unadmitted {
    pub fn why(self) -> &'static str {
        match self {
            Unadmitted::NoMint => {
                "its pack could not be vouched for: no enrolled root, or may not mint"
            }
            Unadmitted::NotRun => {
                "this machine does not run software installed here unless setup allows it \
                 (\"Also software installed here\"), so the install stops here"
            }
        }
    }
}

/// The trailer `mint` makes over the index `head`, kept only when `serves`
/// takes the pack under it, as a run's load does.
pub fn admitted(
    head: &[u8],
    mint: impl FnOnce(&[u8]) -> Option<Vec<u8>>,
    serves: impl FnOnce(&[u8], &[u8]) -> bool,
) -> Result<Vec<u8>, Unadmitted> {
    let trailer = mint(head).ok_or(Unadmitted::NoMint)?;
    match serves(head, &trailer) {
        true => Ok(trailer),
        false => Err(Unadmitted::NotRun),
    }
}
