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

//! The join body: the network, the passphrase and the flags octet.

use super::super::network::SSID_MAX;

/// The longest passphrase a join carries (the saved list's `PASS_MAX`): 63
/// characters or 64 hex digits.
pub const JOIN_PASS_MAX: usize = 64;
/// The largest join body.
pub const JOIN_BODY_MAX: usize = 3 + SSID_MAX + JOIN_PASS_MAX;
/// The AKM suite type of WPA3-Personal (SAE).
pub const AKM_SAE: u8 = 8;

pub(super) const FLAG_WPA3_ONLY: u8 = 1 << 0;
pub(super) const FLAG_HIDDEN: u8 = 1 << 1;
/// net.core's rejoin of a network it lost: refused once the person left it.
pub const FLAG_REJOIN: u8 = 1 << 2;

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
