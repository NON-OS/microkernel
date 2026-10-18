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

//! The station's counters and refusals: the sequence and packet numbers
//! advance with each frame so a nonce is never reused, and an unassociated
//! station or a runt ethernet frame transmits nothing.

use crate::station_tests::{eth, BSSID, OUR, TK};
use nonos_wifi_core::station::{Ccmp, LinkStation};

#[test]
fn an_unassociated_station_transmits_nothing() {
    let mut sta = LinkStation::new(OUR);
    assert!(!sta.is_associated());
    assert!(sta.tx_frame(&eth()).is_none(), "no framing before association");
}

#[test]
fn the_sequence_and_packet_numbers_advance_each_frame() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Software { tk: TK });
    let first = sta.tx_frame(&eth()).expect("tx1");
    let second = sta.tx_frame(&eth()).expect("tx2");

    // Sequence-control high 12 bits carry the sequence number: 0 then 1.
    let seq1 = u16::from_le_bytes([first[22], first[23]]) >> 4;
    let seq2 = u16::from_le_bytes([second[22], second[23]]) >> 4;
    assert_eq!((seq1, seq2), (0, 1), "the sequence number advances per frame");

    // The CCMP header's first packet-number octet advances 1 -> 2.
    assert_eq!(first[24], 1, "first packet number is one");
    assert_eq!(second[24], 2, "packet number advances, so a nonce is never reused");
    assert_ne!(first, second, "identical plaintext yields distinct frames");
}

#[test]
fn a_malformed_ethernet_frame_is_refused() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    assert!(sta.tx_frame(&[0u8; 8]).is_none(), "a runt shorter than an ethernet header is refused");
}
