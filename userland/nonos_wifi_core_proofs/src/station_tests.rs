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

//! Proofs for the shared station data-path engine: a hardware-CCMP station
//! frames plaintext, a software-CCMP station encrypts, and both recover the
//! frame; the counters and refusals are in `station_counter_tests`. No
//! hardware; the AES-CCM core these rest on is proven against RFC 3610 elsewhere.

use nonos_wifi_core::station::{Ccmp, LinkStation};

pub const OUR: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const BSSID: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const DST: [u8; 6] = [0x33, 0x33, 0x00, 0x00, 0x00, 0x16];
pub const TK: [u8; 16] = [
    0xc9, 0x7c, 0x1f, 0x67, 0xce, 0x37, 0x11, 0x85, 0x51, 0x4a, 0x8a, 0x19, 0xf2, 0xbd, 0xd5, 0x2f,
];
const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
pub const FC_PROTECTED: u16 = 0x4000;

pub fn eth() -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&DST);
    f.extend_from_slice(&OUR); // the stack sends with our MAC as source
    f.extend_from_slice(&[0x08, 0x00]);
    f.extend_from_slice(b"a frame that rides the radio");
    f
}

#[test]
fn a_hardware_station_frames_plaintext_and_recovers_it() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    let mpdu = sta.tx_frame(&eth()).expect("tx");

    // The radio does the crypto, so the station must hand it a plaintext frame.
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    assert_eq!(fc & FC_PROTECTED, 0, "hardware CCMP frame is not marked encrypted");
    assert_eq!(&mpdu[24..30], &LLC_SNAP, "body is plaintext LLC/SNAP");

    // The receive path (chip already decrypted) recovers the ethernet frame.
    let back = sta.rx_frame(&mpdu).expect("rx");
    assert_eq!(back, eth(), "the ethernet frame survives the hardware round trip");
}

#[test]
fn a_software_station_encrypts_and_recovers_it() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Software { tk: TK });
    let frame = sta.tx_frame(&eth()).expect("tx");

    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    assert_ne!(fc & FC_PROTECTED, 0, "software CCMP marks the frame protected");
    assert_ne!(&frame[32..38], &LLC_SNAP, "the body is ciphertext, not plaintext LLC/SNAP");

    let back = sta.rx_frame(&frame).expect("rx");
    assert_eq!(back, eth(), "the ethernet frame survives the software round trip");
}
