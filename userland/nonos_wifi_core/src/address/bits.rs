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

//! The bits that make an address the station's own, and a network's
//! security mode as its identity counts it.

/// An address the station made up: individual (bit 0 of the first octet
/// clear) and locally administered (bit 1 set), so it can never be taken
/// for a group address or for a vendor's.
/// The rule is `nonos_mac`'s, the one every network driver uses.
pub fn local_unicast(mut a: [u8; 6]) -> [u8; 6] {
    nonos_mac::apply(&mut a);
    a
}

/// What a network's address is keyed on: a label, the SSID, then the
/// security octet. The octet is one fixed width and always last, so the
/// input reads back to one (SSID, security) pair without a length.
pub fn identity<'a>(ssid: &'a [u8], security: &'a u8) -> [&'a [u8]; 3] {
    [b"join", ssid, core::slice::from_ref(security)]
}

/// The security a request asks for, as a network's identity counts it.
pub const SECURITY_OPEN: u8 = 0;
pub const SECURITY_PASSPHRASE: u8 = 1;
pub const SECURITY_WPA3: u8 = 2;

/// The mode of a request with passphrase `pass`: a WPA3-only request is
/// another network than the same SSID taken under any personal AKM. The
/// AKM the beacon settles is not part of it, so a join that falls back
/// from SAE to PSK stays on one address.
pub fn security_of(wpa3_only: bool, pass: &[u8]) -> u8 {
    match (pass.is_empty(), wpa3_only) {
        (true, _) => SECURITY_OPEN,
        (false, true) => SECURITY_WPA3,
        (false, false) => SECURITY_PASSPHRASE,
    }
}
