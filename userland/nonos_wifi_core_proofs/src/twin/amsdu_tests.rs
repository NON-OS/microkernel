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

//! A-MSDU. This station negotiates no SPP A-MSDU (its RSN capabilities
//! leave both bits clear), so the A-MSDU Present bit of a QoS frame is not
//! under the MIC and anyone may flip it. The station therefore takes no
//! A-MSDU at all: a sealed QoS frame with the bit set is refused, whatever
//! its body reads as, the injected shape included (a first subframe header
//! that reads as LLC/SNAP). The same frame without the bit is taken.

use super::air::{seal, IP};
use super::watch::{delivered, put};
use crate::ap_sim::AA;
use crate::soft::join::{joined, sta_of};
use crate::soft::rig::Station;

/// A QoS data frame from the AP, TID 0, its A-MSDU Present bit `amsdu`.
fn qos(st: &Station, amsdu: bool, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0x88, 0x02, 0, 0];
    f.extend_from_slice(&sta_of(st));
    f.extend_from_slice(&AA);
    f.extend_from_slice(&AA);
    f.extend_from_slice(&[0x10, 0, if amsdu { 0x80 } else { 0 }, 0]);
    f.extend_from_slice(body);
    f
}

#[test]
fn no_amsdu_is_taken_whatever_its_first_subframe_reads_as() {
    let mut st = joined();
    let start = delivered(&st);
    // The injected shape: the subframe's destination and source read as an
    // LLC/SNAP header and a type, so a receiver parsing it as one MSDU
    // would hand on what the attacker put after it.
    let mut evil = IP[..8].to_vec();
    evil.extend_from_slice(&[0, 0, 0, 0, 0, 20]);
    evil.extend_from_slice(&IP);
    for (pn, body) in [(1, evil), (2, IP.to_vec())] {
        put(&mut st, |s| seal(s, &qos(s, true, &body), pn));
    }
    assert_eq!(delivered(&st), start, "no A-MSDU reached the stack");
    put(&mut st, |s| seal(s, &qos(s, false, &IP), 3));
    assert_eq!(delivered(&st), start + 1, "one MSDU is taken");
    // The RSN capabilities of the association request: SPP A-MSDU Capable
    // and Required are bits 10 and 11 (recalled, unverified).
    let req = st.radio().unwrap().sent.iter().find(|f| f[0] == 0x00).unwrap().clone();
    let at = req[28..].windows(2).position(|w| w == [0x30, 0x14]).unwrap() + 28;
    assert_eq!(u16::from_le_bytes([req[at + 20], req[at + 21]]) & 0x0c00, 0, "no SPP");
}
