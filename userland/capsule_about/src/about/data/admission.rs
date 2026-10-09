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

//! How the kernel admitted this window, as its attestation registry records
//! it, and the badge the Overview draws for it. Pure, so the rule is proven on
//! the host (apps_proofs about_tests).

use super::trust::HYBRID_SCHEME;

/// The registry's authority byte for a capsule that ran on a publisher's
/// signature with no proof behind it.
const PUBLISHER: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admission {
    /// Listed, admitted under a proof (the vendor's tree or an enrolled root).
    Proved,
    /// Listed, admitted on a publisher's signature alone.
    SignedOnly,
    /// The registry answered and this window's pid is not in it.
    Missing,
    /// The registry could not be read, or the window has no pid to look for.
    Unread,
}

/// `entries` is the registry as (pid, authority) pairs, `None` when the kernel
/// refused the read.
pub fn of(me: u32, entries: Option<&[(u32, u8)]>) -> Admission {
    let Some(entries) = entries else {
        return Admission::Unread;
    };
    if me == 0 {
        return Admission::Unread;
    }
    match entries.iter().find(|(pid, _)| *pid == me) {
        Some((_, PUBLISHER)) => Admission::SignedOnly,
        Some(_) => Admission::Proved,
        None => Admission::Missing,
    }
}

/// The badge's label, whether it is drawn as a pass, and the line under it.
/// Only a proved admission is a pass; anything the registry did not say is
/// never drawn as one.
pub fn badge(a: Admission) -> (&'static [u8], bool, &'static [u8]) {
    match a {
        Admission::Proved => (b"Verified", true, HYBRID_SCHEME),
        Admission::SignedOnly => (b"Signed, no proof", false, HYBRID_SCHEME),
        Admission::Missing => (b"Not admitted", false, b"not in the attestation registry"),
        Admission::Unread => (b"Unknown", false, b"attestation registry not readable"),
    }
}
