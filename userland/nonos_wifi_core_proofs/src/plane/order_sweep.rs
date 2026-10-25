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

//! The transitions in every order. Over seeded interleavings of join,
//! deauthentication, transmit, receive and idle ticks, a non-EAPOL data
//! frame leaves only while the link reports up, and a received frame is
//! delivered only while it is up. The link state is read before each action
//! and is the oracle; it is up only once the keys are in, and down the
//! moment teardown begins.

use super::drive::{rx, tx, up};
use super::rig::{data_frames, fresh};
use crate::soft::join::{deauth_from_ap, ticks, PASS, SSID};
use crate::soft::wire::connect;
use crate::sweep::rng::{Rng, SEEDS};

#[test]
fn a_data_frame_leaves_only_while_the_link_is_up() {
    let (mut sent_up, mut delivered, mut tore_down) = (0u64, 0u64, 0u64);
    for seed in SEEDS {
        let mut st = fresh();
        let mut rng = Rng::new(seed);
        for _ in 0..400 {
            let before_up = up(&mut st);
            let before_data = data_frames(&st);
            match rng.below(5) {
                0 => {
                    let _ = connect(&mut st, SSID, PASS, 0);
                }
                1 => {
                    if before_up {
                        tore_down += 1;
                    }
                    deauth_from_ap(&mut st);
                    ticks(&mut st, 3);
                }
                2 => {
                    let ok = tx(&mut st, rng.byte()) >= 0;
                    assert_eq!(ok, before_up, "transmit accepted iff the link is up");
                    let grew = data_frames(&st) - before_data;
                    assert!(grew == 0 || before_up, "a data frame left while down");
                    sent_up += grew as u64;
                }
                3 => {
                    if rx(&mut st) {
                        assert!(before_up, "a frame was delivered while down");
                        delivered += 1;
                    }
                }
                _ => ticks(&mut st, 1 + rng.below(8)),
            }
        }
    }
    assert!(sent_up > 0, "the sweep sent data");
    assert!(tore_down > 0, "the sweep tore links down");
    let _ = delivered;
}
