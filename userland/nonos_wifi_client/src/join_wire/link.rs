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

//! The link reply: whether the radio is associated, and with what.

use super::super::network::SSID_MAX;
use super::join_body::AKM_SAE;
use super::tail::{parse_tail, LastJoin, LinkEnded};

/// The driver's association as it reports it.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Link {
    pub associated: bool,
    pub bssid: [u8; 6],
    ssid: [u8; SSID_MAX],
    ssid_len: usize,
    /// The AKM that ran (0 when the driver does not say).
    pub akm: u8,
    /// Why the link last ended, and the last join the driver finished.
    pub ended: LinkEnded,
    pub last_join: Option<LastJoin>,
    /// Message 3's Transition Disable bitmap for this association.
    pub transition_disable: u8,
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
        (link.ended, link.last_join, link.transition_disable) = parse_tail(body, 9 + len);
    }
    Some(link)
}
