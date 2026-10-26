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

//! The order a pack goes in and out in, with the store reached through a
//! `Shelf` so the order is proved on the host (capsule_linux_proofs). A
//! pack a run holds open is neither replaced nor removed: the store answers
//! busy, and nothing of the package changes until the run is done.
//!
//! Every state a crash can leave serves no file that is not the package's:
//! the old pack with its trailer; no pack; the new pack under the old
//! trailer, which does not verify and so serves nothing; or the new pack
//! with its own trailer. A `.new` left over is no pack's name, so nothing
//! loads it, and the next put writes over it.

use alloc::vec::Vec;

use super::paths::{pack_path, trailer_path};
use super::shelf::{Held, Shelf, Unlinked};

/// Where a pack is written whole before it takes the old one's place.
pub fn new_path(package: &str) -> Vec<u8> {
    [pack_path(package).as_slice(), b".new"].concat()
}

/// `pack` in as `package`'s, `trailer` the one minted over its index.
pub fn swap(s: &mut impl Shelf, package: &str, pack: &[u8], trailer: &[u8]) -> Result<(), Held> {
    let (at, new) = (pack_path(package), new_path(package));
    let old = match s.write(&new, pack) {
        true => gone(s, &at),
        false => Err(Held::Unwritten),
    };
    if let Err(why) = old {
        s.unlink(&new);
        return Err(why);
    }
    match s.rename(&new, &at) && s.write(&trailer_path(package), trailer) {
        true => Ok(()),
        false => Err(Held::Unwritten),
    }
}

/// `package`'s pack out, then its trailer and any `.new` a crash left.
pub fn take(s: &mut impl Shelf, package: &str) -> Result<(), Held> {
    gone(s, &pack_path(package))?;
    s.unlink(&trailer_path(package));
    s.unlink(&new_path(package));
    Ok(())
}

fn gone(s: &mut impl Shelf, at: &[u8]) -> Result<(), Held> {
    match s.unlink(at) {
        Unlinked::Gone => Ok(()),
        Unlinked::Busy => Err(Held::InUse),
        Unlinked::Failed => Err(Held::Unwritten),
    }
}
