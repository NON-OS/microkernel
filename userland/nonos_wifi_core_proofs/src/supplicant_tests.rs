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

//! The group key's index survives the four-way handshake. A simulated AP
//! drives the real `Supplicant` through messages 1 and 3 with the GTK KDE
//! naming index 1, then index 2, and the proof holds the supplicant (and the
//! MLME accessor the drivers read) to the index the AP sent. Group-addressed
//! frames name their key by this index and an AP moves it between 1 and 2 on
//! every rekey, so a driver that installs the group key anywhere else stops
//! decrypting broadcast (ARP, DHCP, router advertisements) after the first one.

use crate::supplicant_sim::handshake;

#[test]
fn the_group_key_keeps_the_index_the_ap_sent() {
    assert_eq!(handshake(0x01).gtk_id(), 1);
    assert_eq!(handshake(0x02).gtk_id(), 2, "the index after an AP's first rekey");
    assert_eq!(handshake(0x03).gtk_id(), 3);
}

#[test]
fn only_the_index_bits_are_taken() {
    // Bit 2 of the byte is the Tx flag, not part of the index.
    assert_eq!(handshake(0x04 | 0x02).gtk_id(), 2);
}
