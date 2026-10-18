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
//! Events that carry a list (Vol 4, Part E, 7.7.19 and 7.7.65.2): a sound
//! list reads as built; a count that lies either way, advertising data
//! past 31 octets, or a data length past the event is refused whole.

use super::adv::reports;
use super::rng::each;
use nonos_bt_wire::hci::reports::{adv_reports, completed_packets};
use nonos_bt_wire::WireError;

#[test]
fn advertising_reports_read_as_built_and_lies_are_refused() {
    each(|rng| {
        let (ev, built) = reports(rng);
        let got = adv_reports(&ev).expect("sound");
        assert_eq!(got.len(), built.len());
        for (g, b) in got.iter().zip(&built) {
            assert_eq!((g.event_type, g.addr.0, g.data, g.rssi as u8), (b.0, b.2, &b.3[..], b.4));
        }
        let mut lie = ev.clone();
        lie[0] = lie[0].wrapping_add(if rng.below(2) == 0 { 1 } else { 255 });
        assert!(adv_reports(&lie).is_err(), "a count of {}", lie[0]);
        // The last report's data length: a lie there runs past the event.
        if let Some((_, _, _, data, _)) = built.last() {
            let at = ev.len() - data.len() - 2;
            let mut lie = ev.clone();
            lie[at] = 32 + rng.below(224) as u8;
            assert_eq!(adv_reports(&lie).err(), Some(WireError::TooLong), "{} octets", lie[at]);
            if data.len() < 31 {
                let mut lie = ev.clone();
                lie[at] = (data.len() + 1 + rng.below(31 - data.len())) as u8;
                assert!(adv_reports(&lie).is_err(), "a data length past the event");
            }
        }
    });
}

#[test]
fn completed_packets_read_as_built_and_a_lying_count_is_refused() {
    each(|rng| {
        let n = rng.below(6);
        let pairs: Vec<(u16, u16)> =
            (0..n).map(|_| (rng.next() as u16, rng.next() as u16)).collect();
        let mut ev = vec![n as u8];
        pairs.iter().for_each(|(h, c)| ev.extend([h.to_le_bytes(), c.to_le_bytes()].concat()));
        let want: Vec<_> = pairs.iter().map(|&(h, c)| (h & 0x0fff, c)).collect();
        assert_eq!(completed_packets(&ev), Ok(want));
        ev[0] = ev[0].wrapping_add(if rng.below(2) == 0 { 1 } else { 255 });
        assert!(completed_packets(&ev).is_err(), "a count of {}", ev[0]);
    });
}
