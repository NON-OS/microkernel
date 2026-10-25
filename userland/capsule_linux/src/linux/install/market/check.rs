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

//! What the install role holds a market program's policy to before it vouches
//! for it: exactly these four, and any failure fails the install.
//! A release with no policy is held to `without_box`.

use nonos_linux_policy::{decode, market_ready, LinuxAppPolicy};

use super::super::apps::{wanted, Wanted};
use super::arg::MARKET_LEN;

/// Everything that can be decided before the package is fetched: a canonical
/// policy, market ready, naming the listing and release the market resolved.
pub fn admit(market: &[u8; MARKET_LEN]) -> Result<LinuxAppPolicy, &'static str> {
    let p = decode(&market[..256]).map_err(|_| "its policy is malformed")?;
    if !market_ready(&p) {
        return Err("its policy is not market ready");
    }
    let (listing, release) = (&market[256..288], &market[288..320]);
    if p.listing_id == [0; 32] || p.release_id == [0; 32] {
        return Err("its policy names no listing or release");
    }
    if p.listing_id[..] != listing[..] || p.release_id[..] != release[..] {
        return Err("its policy names another listing or release");
    }
    Ok(p)
}

/// The rest, once the program it starts as is in the store.
pub fn names_program(p: &LinuxAppPolicy, image: &[u8]) -> bool {
    p.program_blake3 == *blake3::hash(image).as_bytes()
}

/// The first word on an install, before anything is fetched: the policy a
/// market release's `m` names, held to `admit`; or, for "-", none, which
/// only a shipped tier may have (`without_box`).
pub fn gate(
    pkg: &str,
    market: Option<&[u8; MARKET_LEN]>,
) -> Result<Option<LinuxAppPolicy>, &'static str> {
    match market {
        Some(m) => admit(m).map(Some),
        None => without_box(pkg).map(|()| None),
    }
}

/// A release with no policy of its own ("-") installs only a tier this
/// personality ships, whose box is its own signed table's. A package fetched
/// from a family's index never runs without its release's box.
pub fn without_box(pkg: &str) -> Result<(), &'static str> {
    match wanted(pkg) {
        Wanted::Tier(_) => Ok(()),
        Wanted::UnknownTier | Wanted::Package => {
            Err("a package from an index needs its release's box")
        }
    }
}
