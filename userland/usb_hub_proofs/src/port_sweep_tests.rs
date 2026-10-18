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

//! Port GET_STATUS answers of any four bytes, and answers cut short: the
//! decode reads only the bits the rule names, and a short answer is none.

use nonos_usb_hub::{decode_port_status, port_status, HubPort};

use crate::sweep_answer::Answers;
use crate::sweep_rng::{rounds, Rng};

/// wPortStatus and wPortChange as USB 2.0 table 11-21 and 11-22 and USB
/// 3.2 table 10-10 lay them out (as recalled, unverified): connection,
/// enable and reset at bits 0, 1 and 4; power at 8, or at 9 on a
/// SuperSpeed hub; a USB 2 port's low speed at 9 ahead of high at 10.
fn rule(b: [u8; 4], ss: bool) -> HubPort {
    let (st, ch) = (u16::from_le_bytes([b[0], b[1]]), u16::from_le_bytes([b[2], b[3]]));
    let bit = |w: u16, n: u32| w >> n & 1 == 1;
    let speed = match (ss, bit(st, 9), bit(st, 10)) {
        (true, _, _) => 4,
        (false, true, _) => 2,
        (false, false, true) => 3,
        _ => 1,
    };
    HubPort {
        connected: bit(st, 0),
        enabled: bit(st, 1),
        resetting: bit(st, 4),
        powered: bit(st, if ss { 9 } else { 8 }),
        speed,
        connect_changed: bit(ch, 0),
        reset_changed: bit(ch, 4),
    }
}

#[test]
fn any_four_bytes_decode_by_the_rule_and_a_short_answer_is_none() {
    let mut short = 0u32;
    for seed in 1..rounds(100_000) {
        let mut r = Rng::new(seed);
        let b = (r.next() as u32).to_le_bytes();
        let ss = r.next() & 1 == 1;
        assert_eq!(decode_port_status(b, ss), rule(b, ss), "seed {seed}: {b:?}");
        let n = r.range(0, 5).min(4);
        let mut host = Answers { bytes: b, n };
        let got = port_status(&mut host, 1, r.range(1, 15) as u8, ss);
        assert_eq!(got, (n == 4).then(|| rule(b, ss)), "seed {seed}: {n} bytes");
        short += (n < 4) as u32;
    }
    assert!(cfg!(miri) || short >= 50_000, "a short answer came only {short} times");
}
