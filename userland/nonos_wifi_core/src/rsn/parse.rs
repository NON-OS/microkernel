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

use super::suite::{selector, AKM_PSK, AKM_PSK_SHA256, AKM_SAE, CAP_MFPC, CAP_MFPR, CIPHER_CCMP};

/// The parts of an access point's RSN element the station selects from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rsne {
    /// The group data cipher suite.
    pub group: u32,
    /// Whether CCMP-128 is among the pairwise cipher suites.
    pub pairwise_ccmp: bool,
    pub akm_psk: bool,
    pub akm_psk_sha256: bool,
    pub akm_sae: bool,
    /// The RSN Capabilities field.
    pub caps: u16,
    /// The group management cipher suite, when the element names one.
    pub group_mgmt: Option<u32>,
}

impl Rsne {
    /// Management frame protection capable.
    pub fn mfpc(&self) -> bool {
        self.caps & CAP_MFPC != 0
    }

    /// Management frame protection required.
    pub fn mfpr(&self) -> bool {
        self.caps & CAP_MFPR != 0
    }
}

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
    for _ in 0..r.u16()? {
        match r.suite()? {
            AKM_PSK => out.akm_psk = true,
            AKM_PSK_SHA256 => out.akm_psk_sha256 = true,
            AKM_SAE => out.akm_sae = true,
            _ => {}
        }
    }
    if r.done() {
        return Some(out);
    }
    out.caps = r.u16()?;
    if r.done() {
        return Some(out);
    }
    let pmkids = r.u16()? as usize;
    r.skip(pmkids.checked_mul(16)?)?;
    if r.remaining() >= 4 {
        out.group_mgmt = Some(r.suite()?);
    }
    Some(out)
}

// A bounded little-endian cursor over the element body.
struct Reader<'a> {
    b: &'a [u8],
    off: usize,
}

impl Reader<'_> {
    fn remaining(&self) -> usize {
        self.b.len() - self.off
    }
    fn done(&self) -> bool {
        self.off >= self.b.len()
    }
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let end = self.off.checked_add(n)?;
        let s = self.b.get(self.off..end)?;
        self.off = end;
        Some(s)
    }
    fn skip(&mut self, n: usize) -> Option<()> {
        self.take(n).map(|_| ())
    }
    fn u16(&mut self) -> Option<u16> {
        let s = self.take(2)?;
        Some(u16::from_le_bytes([s[0], s[1]]))
    }
    fn suite(&mut self) -> Option<u32> {
        let s = self.take(4)?;
        Some(selector([s[0], s[1], s[2], s[3]]))
    }
}
