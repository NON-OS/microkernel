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

//! A station that left receive to the chip (`Ccmp::SoftwareTxHwRx`) takes
//! the chip's word, since such a chip may strip the MIC it checked and its
//! driver fills the gap; the packet number is still the station's to hold.
//! The same frame to a station that decrypts receive itself is refused.

use crate::receive::ccmp_tests::tk;
use crate::receive::chip_tests::chip;
use crate::receive::frame::{from_ap, station, AP, STA};
use crate::receive::join::ip_body;
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};

// The chip's frame with its MIC gone and eight zero octets in its place.
fn stripped(pn: u64) -> Vec<u8> {
    let mut f = chip(&from_ap(STA, &ip_body(b"hwrx"), Some(&tk()), pn, 0, true), &ip_body(b"hwrx"));
    let n = f.len();
    f[n - 8..].fill(0);
    f
}

#[test]
fn a_station_that_left_receive_to_the_chip_takes_its_word_with_replay_held() {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::SoftwareTxHwRx { tk: tk() });
    assert!(matches!(s.receive(&stripped(9), true), Rx::Ethernet(_)), "the chip checked it");
    assert!(matches!(s.receive(&stripped(9), true), Rx::Dropped(RxDrop::Replay)));
    assert!(matches!(s.receive(&stripped(8), true), Rx::Dropped(RxDrop::Replay)));
    assert!(matches!(s.receive(&stripped(10), true), Rx::Ethernet(_)));
}

#[test]
fn a_station_that_decrypts_receive_itself_refuses_the_same_frame() {
    let mut s = station();
    assert!(matches!(s.receive(&stripped(9), true), Rx::Dropped(RxDrop::Undecryptable)));
}
