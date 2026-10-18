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

//! A station keyed `Ccmp::Hardware` hands the chip plaintext to encrypt,
//! so nothing goes until the driver says the chip holds the keys, and
//! nothing again once it says the chip lost them (a firmware restart, a key
//! cache cleared, an install never answered), until they are back. EAPOL a
//! handshake sends before any key is in place still goes in the clear;
//! software protection never waits.

use nonos_wifi_core::station::{Ccmp, LinkStation};

const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const BSSID: [u8; 6] = [0x02, 0, 0, 0, 0, 0xaa];

fn eth() -> Vec<u8> {
    [&BSSID[..], &OUR, &[0x08, 0x00], b"held until keyed"].concat()
}

#[test]
fn a_hardware_link_sends_nothing_while_the_chip_holds_no_keys() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    assert_eq!(sta.tx_frame(&eth()), None, "no key install answered yet");
    assert_eq!(sta.eapol_frame(&[1, 3, 0, 0], true), None, "nor a protected EAPOL");
    assert!(sta.eapol_frame(&[1, 3, 0, 0], false).is_some(), "the handshake's own");
    sta.chip_keys(true);
    assert!(sta.tx_frame(&eth()).is_some());
    sta.chip_keys(false);
    assert_eq!(sta.tx_frame(&eth()), None, "a restart took the keys");
    sta.chip_keys(true);
    assert!(sta.tx_frame(&eth()).is_some(), "until they are in again");
    sta.associate(BSSID, Ccmp::Hardware);
    assert_eq!(sta.tx_frame(&eth()), None, "a new association waits again");
    sta.deassociate();
    sta.chip_keys(true);
    assert_eq!(sta.tx_frame(&eth()), None, "and nothing goes unassociated");
}

#[test]
fn software_protection_never_waits_on_the_chip() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Software { tk: [7; 16] });
    sta.chip_keys(false);
    let f = sta.tx_frame(&eth()).expect("sealed by the station");
    assert_ne!(f[1] & 0x40, 0, "protected");
}
