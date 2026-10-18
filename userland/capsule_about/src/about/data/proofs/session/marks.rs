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

//! Each check's mark: the boot chain, admission, proof and the route.

use super::super::census::Census;
use super::verdict::all;
use nonos_route_proof::RouteVerdict;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    Holds,
    Broken,
    Unknown,
}

/// The bootloader's record as the kernel reports it: the kernel signature, the
/// attestation, and the kernel's STARK proof.
#[derive(Clone, Copy, Debug)]
pub struct Boot {
    pub signature: Mark,
    pub attestation: Mark,
    pub proof: Mark,
}

pub fn boot_mark(b: &Boot) -> Mark {
    all([b.signature, b.attestation, b.proof])
}

/// Every process holding authority was admitted (`None`: the kernel did not
/// let this window read the table or the registry).
pub fn admitted_mark(c: Option<&Census>) -> Mark {
    match c {
        None => Mark::Unknown,
        Some(c) if c.unadmitted > 0 => Mark::Broken,
        Some(c) if c.admitted == 0 => Mark::Unknown,
        Some(_) => Mark::Holds,
    }
}

/// Every admitted capsule was proven, not only signed.
pub fn proven_mark(c: Option<&Census>) -> Mark {
    match c {
        None => Mark::Unknown,
        Some(c) if c.signed_only > 0 => Mark::Broken,
        Some(c) if c.admitted == 0 => Mark::Unknown,
        Some(_) => Mark::Holds,
    }
}

pub fn route_mark(r: RouteVerdict) -> Mark {
    match r {
        RouteVerdict::Anonymous(_) => Mark::Holds,
        RouteVerdict::Exposed => Mark::Broken,
        RouteVerdict::NotEstablished(..) | RouteVerdict::Unknown => Mark::Unknown,
    }
}
