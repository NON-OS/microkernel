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

use crate::roots::{chunk0, chunk1, chunk2, chunk3, extra, store};

/*
 * A `devnet` build (development images only, P5) also trusts the local test
 * chain's throwaway root, by both of the ways a chain can end at a root: the
 * root served and pinned by its key's hash, or named as the top's issuer.
 * Without the feature the module is not compiled, so a release cannot know it.
 */
pub fn is_trusted_spki_hash(h: &[u8; 32]) -> bool {
    chunk0::ROOTS_0.iter().any(|r| r == h)
        || chunk1::ROOTS_1.iter().any(|r| r == h)
        || chunk2::ROOTS_2.iter().any(|r| r == h)
        || chunk3::ROOTS_3.iter().any(|r| r == h)
        || extra::EXTRA_ROOTS.iter().any(|r| r == h)
        || dev_root(h)
}

// Return the SPKI DER of a trusted root whose SubjectDN equals `subject`.
pub fn find_spki_by_subject(subject: &[u8]) -> Option<&'static [u8]> {
    store::find_spki_by_subject(subject).or_else(|| dev_spki(subject))
}

#[cfg(feature = "devnet")]
fn dev_root(h: &[u8; 32]) -> bool {
    crate::roots::dev::is_dev_root(h)
}

#[cfg(feature = "devnet")]
fn dev_spki(subject: &[u8]) -> Option<&'static [u8]> {
    crate::roots::dev::dev_spki_by_subject(subject)
}

#[cfg(not(feature = "devnet"))]
fn dev_root(_: &[u8; 32]) -> bool {
    false
}

#[cfg(not(feature = "devnet"))]
fn dev_spki(_: &[u8]) -> Option<&'static [u8]> {
    None
}
