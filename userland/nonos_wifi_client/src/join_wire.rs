/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The join and link messages between the client and the drivers, as pure
//! encoding and parsing over untrusted bytes (checked in wifi_panel_proofs).
//!
//! A join body is `[ssid_len][ssid][pass_len][pass][flags]`. The flags octet
//! carries what the saved list knows about the network: bit 0, it was saved
//! as WPA3, so the driver joins it with SAE or not at all (never WPA2, which
//! an attacker offering a WPA2-only copy of the network would otherwise get);
//! bit 1, it is hidden, so the driver may name it in a probe request. A driver
//! that predates the octet ignores it.
//!
//! The connect reply body is `[status i32][sent u32][recv u32][data u32]
//! [eapol u32][probe u32][deauth u32][to_us u32][state u8]`, then `[akm u8]
//! [ap_code u16]` from a driver that runs WPA3: the AKM that ran (2 WPA2-PSK,
//! 6 PSK-SHA256, 8 WPA3-SAE) and the access point's status or reason code.
//! The link reply body is `[associated u8][bssid 6][ssid_len u8][ssid]`, then
//! `[akm u8]` from such a driver. Every field is read only when the reply
//! reaches it; a shorter reply reads as zero there.

use super::network::SSID_MAX;

/// The longest passphrase a join carries (the saved list's `PASS_MAX`): 63
/// characters or 64 hex digits.
pub const JOIN_PASS_MAX: usize = 64;
/// The largest join body.
pub const JOIN_BODY_MAX: usize = 3 + SSID_MAX + JOIN_PASS_MAX;
/// The AKM suite type of WPA3-Personal (SAE).
pub const AKM_SAE: u8 = 8;

const FLAG_WPA3_ONLY: u8 = 1 << 0;
const FLAG_HIDDEN: u8 = 1 << 1;

/// What a join may do, from how the network was saved.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct JoinFlags {
    /// Saved as WPA3: SAE or nothing.
    pub wpa3_only: bool,
    /// Hidden: a probe request may name it.
    pub hidden: bool,
}

impl JoinFlags {
    /// The flags octet.
    pub fn octet(self) -> u8 {
        (if self.wpa3_only { FLAG_WPA3_ONLY } else { 0 }) | if self.hidden { FLAG_HIDDEN } else { 0 }
    }
}

/// Encode a join body into `out`, returning its length. An SSID or
/// passphrase longer than the wire holds is cut to it, as the drivers read.
pub fn encode_join(ssid: &[u8], pass: &[u8], flags: JoinFlags, out: &mut [u8; JOIN_BODY_MAX]) -> usize {
    let (sl, pl) = (ssid.len().min(SSID_MAX), pass.len().min(JOIN_PASS_MAX));
    out[0] = sl as u8;
    out[1..1 + sl].copy_from_slice(&ssid[..sl]);
    out[1 + sl] = pl as u8;
    out[2 + sl..2 + sl + pl].copy_from_slice(&pass[..pl]);
    out[2 + sl + pl] = flags.octet();
    3 + sl + pl
}

/// A join's result: the driver status code (0 joined, negative a named
/// failure, see `join_text`), how far the handshake got, the AKM that ran
/// and the access point's own status or reason code.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

/// Parse a connect reply body, or `None` when it does not reach the status.
pub fn parse_connect_reply(body: &[u8]) -> Option<ConnectResult> {
    let code = le32(body, 0)? as i32;
    if body.len() < 33 {
        return Some(ConnectResult { code, ..Default::default() });
    }
    Some(ConnectResult {
        code,
        sent: le32(body, 4)?,
        recv: le32(body, 8)?,
        data: le32(body, 12)?,
        eapol: le32(body, 16)?,
        probe: le32(body, 20)?,
        deauth: le32(body, 24)?,
        to_us: le32(body, 28)?,
        state: body[32],
        akm: body.get(33).copied().unwrap_or(0),
        ap_code: body.get(34..36).map_or(0, |b| u16::from_le_bytes([b[0], b[1]])),
    })
}

/// The driver's association as it reports it.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Link {
    pub associated: bool,
    pub bssid: [u8; 6],
    ssid: [u8; SSID_MAX],
    ssid_len: usize,
    /// The AKM that ran (0 when the driver does not say).
    pub akm: u8,
}

impl Link {
    /// The network the radio is associated with; empty when it is not.
    pub fn ssid(&self) -> &[u8] {
        &self.ssid[..self.ssid_len]
    }

    /// Associated with `ssid` through WPA3-SAE.
    pub fn joined_with_sae(&self, ssid: &[u8]) -> bool {
        self.associated && self.ssid() == ssid && self.akm == AKM_SAE
    }
}

/// Parse a link reply body, or `None` when it is too short to be one (a
/// driver that cannot join answers with only a status code).
pub fn parse_link(body: &[u8]) -> Option<Link> {
    let fixed = body.get(..8)?;
    let len = (fixed[7] as usize).min(SSID_MAX).min(body.len() - 8);
    let mut link = Link { associated: fixed[0] != 0, ..Default::default() };
    link.bssid.copy_from_slice(&fixed[1..7]);
    link.ssid[..len].copy_from_slice(&body[8..8 + len]);
    link.ssid_len = len;
    // The AKM follows a whole SSID only; a cut-short one leaves it unread.
    if len == fixed[7] as usize {
        link.akm = body.get(8 + len).copied().unwrap_or(0);
    }
    Some(link)
}

fn le32(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at.checked_add(4)?)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}
