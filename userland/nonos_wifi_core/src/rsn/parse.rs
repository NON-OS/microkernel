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

//! Parse the RSN element an access point advertises (IEEE Std 802.11-2020,
//! 9.4.2.24): version, group cipher, the pairwise cipher and AKM suite lists,
//! RSN capabilities, the PMKID list and the group management cipher. Every
//! field after the version is optional and takes its default when the element
//! ends before it; a list whose count runs past the element is malformed and
//! the element is refused, so a beacon cannot make the station read past it.

use super::parse_tail::tail;
use super::reader::Reader;
pub use super::rsne::Rsne;
use super::suite::CIPHER_CCMP;

/// Parse an RSN element's content (the bytes after its id and length).
/// Returns `None` for a version other than 1 or a suite list that runs past
/// the element.
pub fn parse_rsne(body: &[u8]) -> Option<Rsne> {
    let mut r = Reader { b: body, off: 0 };
    if r.u16()? != 1 {
        return None;
    }
    // Defaults for an element that ends early (9.4.2.24.1): CCMP for both
    // ciphers and 802.1X for the AKM, which this station does not run, so no
    // AKM flag is set until the element lists one.
    let mut out = Rsne {
        group: CIPHER_CCMP,
        pairwise_ccmp: true,
        akm_psk: false,
        akm_psk_sha256: false,
        akm_sae: false,
        caps: 0,
        group_mgmt: None,
    };
    if r.done() {
        return Some(out);
    }
    out.group = r.suite()?;
    if r.done() {
        return Some(out);
    }
    out.pairwise_ccmp = false;
    for _ in 0..r.u16()? {
        if r.suite()? == CIPHER_CCMP {
            out.pairwise_ccmp = true;
        }
    }
    if r.done() {
        return Some(out);
    }
    tail(&mut r, &mut out)?;
    Some(out)
}
