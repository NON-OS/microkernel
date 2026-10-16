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
//! A protected frame the chip says it decrypted, its CCMP or GCMP header
//! and its MIC left as they came over the air. Where the station decrypts
//! receive itself (`Ccmp::Software`, `Ccmp::Gcmp256`), the chip's word is
//! not enough: the plaintext, sealed again at the frame's own packet number,
//! must give that MIC. A station that left receive to the chip
//! (`Ccmp::Hardware`, `Ccmp::SoftwareTxHwRx`) takes it at its word, since
//! such a chip may strip the MIC it checked.

use super::cipher::seal;
use super::{Ccmp, LinkStation};
use crate::dot11::ccmp::{CCMP_HDR_LEN, MIC_LEN};
use crate::dot11::gcmp::GCMP_MIC_LEN;
use crate::dot11::header::FC_PROTECTED;
use crate::rsn::TemporalKey;
use alloc::vec::Vec;

impl LinkStation {
    /// Whether this station, not the chip, checks what it receives.
    pub(super) fn checks_rx(&self) -> bool {
        matches!(self.ccmp, Ccmp::Software { .. } | Ccmp::Gcmp256 { .. })
    }
}

/// The plaintext body of `frame`, its MAC header `hdr_len` octets,
/// checked under `key` when one is given.
pub(super) fn chip_body(
    frame: &[u8],
    hdr_len: usize,
    pn: u64,
    key: Option<&TemporalKey>,
) -> Option<Vec<u8>> {
    let mic = match key {
        Some(TemporalKey::Gcmp256(_)) => GCMP_MIC_LEN,
        _ => MIC_LEN,
    };
    let end = frame.len().checked_sub(mic)?;
    let body = frame.get(hdr_len + CCMP_HDR_LEN..end)?;
    let Some(key) = key else { return Some(body.to_vec()) };
    let mut plain = frame.get(..hdr_len)?.to_vec();
    plain[1] &= !((FC_PROTECTED >> 8) as u8);
    plain.extend_from_slice(body);
    let again = seal(&plain, pn, key)?;
    let want = again.get(again.len().checked_sub(mic)?..)?;
    // Every octet compared, so the time taken says nothing of where it differs.
    let differ = frame[end..].iter().zip(want).fold(0u8, |d, (a, b)| d | (a ^ b));
    (differ == 0).then(|| body.to_vec())
}
