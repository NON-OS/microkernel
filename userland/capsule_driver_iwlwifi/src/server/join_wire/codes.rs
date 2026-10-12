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

//! The flags, the header and the status codes the join messages share.

pub(super) const FLAG_WPA3_ONLY: u8 = 1 << 0;

pub(super) const FLAG_HIDDEN: u8 = 1 << 1;

pub(super) const SSID_MAX: usize = 32;

/// The control family header.
pub const WIFI_HDR: usize = 10;

/// The connect reply after the header.
pub const CONNECT_REPLY: usize = 36;

/// The radio is down, the request malformed, or the firmware failed a join
/// command.
pub const CODE_BAD_REQUEST: i32 = -1;

/// The network was not heard on any channel.
pub const CODE_NOT_FOUND: i32 = -2;

/// The pairwise or the group key did not go into the firmware.
pub const CODE_PAIRWISE_KEY: i32 = -3;

pub const CODE_GROUP_KEY: i32 = -4;

/// The access point refused authentication or association.
pub const CODE_REFUSED: i32 = -5;

/// The handshake did not finish (a wrong WPA2 passphrase ends here).
pub const CODE_TIMED_OUT: i32 = -6;

/// Saved as WPA3, the network now offers only WPA2: not joined.
pub const CODE_DOWNGRADE: i32 = -7;

/// Open, TKIP, Enterprise or 802.11n-only.
pub const CODE_UNSUPPORTED: i32 = -8;

/// Not 8 to 63 characters or 64 hex digits.
pub const CODE_BAD_PASSPHRASE: i32 = -9;

/// WPA3: the access point's SAE confirm did not verify.
pub const CODE_WRONG_PASSWORD: i32 = -10;

/// Message 3 did not carry the beacon's security elements.
pub const CODE_IE_MISMATCH: i32 = -11;

/// No randomness for the nonce and SAE secrets.
pub const CODE_NO_ENTROPY: i32 = -12;
