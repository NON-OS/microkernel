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

//! Key reinstallation. The twin replays, reorders and repeats every
//! handshake message (message 3 as it was, messages 3 and 1 sent again,
//! group message 1 with the key in use, a new key, or one rotated back to)
//! between the AP's data, replays of it, and the station's own sends. The
//! station's transmit packet number only rises, and no frame is taken twice.

use super::air::{data, four_way, group};
use super::watch::{delivered, put, send, sent_pns};
use crate::ap_sim::GTK;
use crate::soft::join::joined;
use crate::sweep::rng::{Rng, SEEDS};

const KEYS: [[u8; 16]; 3] = [GTK, [0x7e; 16], [0x3c; 16]];

#[test]
fn no_handshake_replayed_reordered_or_repeated_resets_a_counter() {
    for seed in SEEDS {
        let (mut st, mut rng) = (joined(), Rng::new(seed));
        let (mut rc, mut pn, mut want) = (2u8, 0u64, delivered(&st));
        // The group key in use (KEYS index, key index); each key index's key and last PN.
        let (mut at, mut slots, mut gpn) = ((0usize, 1u8), [None, Some(0), None], [0u64; 3]);
        for round in 0..400 {
            let before = delivered(&st);
            let (third, k, i) = (rng.below(2) == 0, rng.below(3), 1 + rng.below(2) as u8);
            let (old, gold) = (
                1 + rng.below(pn.max(1) as usize),
                1 + rng.below(gpn[at.1 as usize].max(1) as usize),
            );
            let g = (&KEYS[at.0], at.1);
            match rng.below(9) {
                0 => put(&mut st, |s| four_way(s, true, 2)),
                1 if rc < 250 => (rc += 1, put(&mut st, |s| four_way(s, third, rc))).1,
                2 => put(&mut st, |s| group(s, rc, g.0, g.1)),
                3 if rc < 250 => {
                    rc += 1;
                    at = (k, i);
                    if slots[usize::from(i)] != Some(k) {
                        (slots[usize::from(i)], gpn[usize::from(i)]) = (Some(k), 0);
                    }
                    put(&mut st, |s| group(s, rc, &KEYS[k], i));
                }
                4 => (pn += 1, put(&mut st, |s| data(s, pn, None)), want += 1).1,
                5 if pn > 0 => put(&mut st, |s| data(s, old as u64, None)),
                6 => {
                    gpn[usize::from(at.1)] += 1;
                    put(&mut st, |s| data(s, gpn[usize::from(at.1)], Some(g)));
                    want += 1;
                }
                7 if gpn[usize::from(at.1)] > 0 => put(&mut st, |s| data(s, gold as u64, Some(g))),
                _ => send(&mut st),
            }
            assert!(delivered(&st) - before <= 1, "seed {seed:x} round {round}: once");
            let pns = sent_pns(&st);
            assert!(pns.windows(2).all(|w| w[0] < w[1]), "seed {seed:x} round {round}: {pns:?}");
        }
        assert_eq!(delivered(&st), want, "seed {seed:x}: every fresh frame, and no replay");
        assert!(st.session().is_some() && sent_pns(&st).len() > 20, "the link held and sent");
    }
}
