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
//! The chip's word on a GCMP-256 link, whose MIC is 16 octets: checked the
//! same way under the station's own key.

use crate::receive::frame::{from_ap, AP, STA};
use crate::receive::join::ip_body;
use nonos_wifi_core::dot11::gcmp::encrypt;
use nonos_wifi_core::station::{Ccmp, LinkStation, Rx, RxDrop};

const TK: [u8; 32] = [0x3c; 32];

fn at(pn: u64) -> Vec<u8> {
    let plain = from_ap(STA, &ip_body(b"gcmp"), None, 0, 0, true);
    let sealed = encrypt(&plain, pn, &TK).unwrap();
    [&sealed[..26 + 8], &ip_body(b"gcmp")[..], &sealed[sealed.len() - 16..]].concat()
}

#[test]
fn a_gcmp_frame_said_decrypted_is_checked_under_the_stations_key() {
    let mut s = LinkStation::new(STA);
    s.associate(AP, Ccmp::Gcmp256 { tk: TK });
    let honest = at(40);
    let mut lie = honest.clone();
    lie[honest.len() - 1] ^= 1;
    assert!(matches!(s.receive(&lie, true), Rx::Dropped(RxDrop::Undecryptable)));
    let Rx::Ethernet(eth) = s.receive(&honest, true) else { panic!("delivered") };
    assert_eq!(&eth[14..], b"gcmp", "no MIC octets trail it");
}
