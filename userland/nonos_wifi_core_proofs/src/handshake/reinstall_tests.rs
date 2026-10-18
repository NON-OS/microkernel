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

//! E-68: from Connected, no hostile handshake frame reinstalls a key or
//! rolls a counter back (IEEE Std 802.11-2020, 12.7.6, 12.7.7). The group
//! key changes only on a genuinely new key — never a retransmit or replay;
//! the EAPOL replay counter only rises, an equal or older one never taken;
//! the pairwise key is never replaced; a forged MIC installs nothing. The
//! three breaks: reinstall on a retransmitted message 3 (new_group_key with
//! no change), an equal counter accepted, a key installed before the MIC.

use super::reinstall_sim::{act, connected};
use crate::sweep::rng::{Rng, SEEDS};
use nonos_wifi_core::wpa::supplicant::State;

#[test]
fn a_hostile_ap_reinstalls_no_key_and_rolls_no_counter_back() {
    for seed in SEEDS {
        let (mut sup, ptk) = connected();
        let mut rng = Rng::new(seed);
        let (tk0, mut max) = (sup.tk().to_vec(), 2u8);
        for round in 0..300 {
            let before = (sup.gtk().to_vec(), sup.gtk_id());
            let (f, c, forged, governed) = act(&mut rng, &sup, &ptk, max);
            let out = sup.step(&f);
            let accepted = out.reply.is_some();
            let changed = before != (sup.gtk().to_vec(), sup.gtk_id());
            assert_eq!(sup.state(), State::Connected, "seed {seed:x} round {round}");
            assert_eq!(sup.tk(), &tk0[..], "the pairwise key was replaced");
            assert_eq!(out.new_group_key, changed, "a key reinstalled or a change missed");
            if forged {
                assert!(!accepted && !changed, "a forged MIC was acted on");
            }
            if governed {
                if c <= max {
                    assert!(!accepted, "an equal or older counter was taken");
                }
                if accepted {
                    assert!(c > max, "a stale counter was taken");
                    max = c;
                }
            }
        }
    }
}
