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
//! Random and cut SAE frames never panic or accept. Two kinds are acted on,
//! as 802.11 has it, though nothing authenticates them: a commit that parses
//! is taken as the AP's, and an anti-clogging request is answered with its
//! token. Either may leave the exchange to the driver's deadline, never
//! accepted, since a forger cannot confirm without the password.

use nonos_wifi_core::sae::{SaeState, SaeStep};

use super::discard::{finish, pair};

#[test]
fn random_and_cut_sae_frames_never_accept() {
    let mut x = 0x2545_f491_4f6c_dd1du64;
    let mut next = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    let (mut s0, mut a0) = pair();
    assert!(finish(&mut s0, &mut a0));
    for round in 0..400 {
        let (mut sta, mut ap) = pair();
        let (_, _, own) = sta.commit_frame();
        for _ in 0..8 {
            let len = (next() % 140) as usize;
            let mut body: Vec<u8> = (0..len).map(|_| next() as u8).collect();
            // Half the frames start as this station's own commit, then cut or flipped.
            if round % 2 == 0 {
                body = own[..len.min(own.len())].to_vec();
                if let Some(b) = body.get_mut(len / 3) {
                    *b ^= 1 << (round % 8);
                }
            }
            let seq = 1 + (next() % 3) as u16;
            let status = [0, 1, 76, 77, 126, next() as u16][(next() % 6) as usize];
            assert!(!matches!(sta.on_frame(seq, status, &body), SaeStep::Accepted { .. }));
            assert_ne!(sta.state(), SaeState::Accepted);
        }
        if finish(&mut sta, &mut ap) {
            assert_eq!(sta.pmk(), s0.pmk(), "round {round}: keyed only with the AP");
        } else {
            assert!(sta.pmk().is_none(), "round {round}: stalled, never keyed");
        }
    }
}
