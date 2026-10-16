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

//! IEEE 802.11 MAC frame header, pure encoding with no hardware. Every
//! management frame the association MLME sends (probe request, authentication,
//! association request) and every EAPOL frame the WPA2 handshake exchanges is
//! built on this header. The field layout is the IEEE 802.11 standard and is
//! stable across chip families, so this is correct by construction and is
//! checked by the `iwlwifi_proofs` crate rather than needing hardware.

pub type MacAddr = [u8; 6];

/// The all-ones address: a wildcard receiver/BSSID for a scanning probe.
pub const BROADCAST: MacAddr = [0xFF; 6];

// Frame Control type field (bits 2-3 of octet 0).
pub const TYPE_MGMT: u8 = 0;

pub const TYPE_DATA: u8 = 2;

// Management-frame subtypes (bits 4-7 of octet 0) the driver builds or parses.
pub const SUBTYPE_ASSOC_REQ: u8 = 0;

pub const SUBTYPE_ASSOC_RESP: u8 = 1;

pub const SUBTYPE_REASSOC_REQ: u8 = 2;

pub const SUBTYPE_REASSOC_RESP: u8 = 3;

pub const SUBTYPE_PROBE_REQ: u8 = 4;

pub const SUBTYPE_PROBE_RESP: u8 = 5;

pub const SUBTYPE_BEACON: u8 = 8;

pub const SUBTYPE_AUTH: u8 = 11;

/// A management or data MAC header without QoS or a fourth address is 24 bytes:
/// frame control (2), duration (2), addr1/2/3 (6 each), sequence control (2).
pub const MAC_HEADER_LEN: usize = 24;

// Frame Control flag bits (octet 1). ToDS/FromDS select the address layout;
// both set means a four-address frame. Protected marks a CCMP-encrypted body.
pub const FC_TO_DS: u16 = 0x0100;

pub const FC_FROM_DS: u16 = 0x0200;

pub const FC_PROTECTED: u16 = 0x4000;
