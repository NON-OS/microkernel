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
//! A responder that offers any key size but 16 (the only one this host
//! takes), or a public key off the curve or past the field (Vol 3, Part H,
//! 2.3.4 and 2.3.5.6.1), leaves pairing failed with no key.

use crate::sim::{Peer, Run};
use crate::sweep_tests::{draw, SEEDS};
use nonos_bt_smp::method::IoCap;
use nonos_bt_smp::pairing::Outcome;
use nonos_bt_smp::pdu::*;

fn failed(run: &mut Run, reason: u8, what: &str) {
    assert_eq!(run.go(), Some(Outcome::Failed { reason, by_peer: false }), "{what}");
    let keyed = |o: &Outcome| matches!(o, Outcome::Encrypt(_) | Outcome::Paired(_));
    assert!(!run.outcomes.iter().any(keyed), "{what}: no key");
}

#[test]
fn a_responders_key_size_other_than_16_ends_pairing_with_no_key() {
    for size in (0..=255u8).filter(|&k| k != 16) {
        let mut peer = Peer::new(IoCap::KeyboardDisplay, true);
        peer.ours.max_key_size = size;
        let reason =
            if (7..=16).contains(&size) { ENCRYPTION_KEY_SIZE } else { INVALID_PARAMETERS };
        failed(&mut Run::new(peer), reason, &format!("key size {size}"));
    }
}

#[test]
fn a_public_key_off_the_curve_or_past_the_field_ends_pairing() {
    for mut s in SEEDS {
        for round in 0..6 {
            let mut peer = Peer::new(IoCap::KeyboardDisplay, true);
            let (x, y) = peer.keys.public();
            let mut key: [u8; 64] = [le(x), le(y)].concat().try_into().unwrap();
            if round == 0 {
                key[..32].fill(0xff);
            } else {
                // For one x the curve holds y and p - y, of opposite parity
                // as p is odd; a flip of any bit but the lowest keeps it.
                let k = 1 + (draw(&mut s) % 255) as usize;
                key[32 + k / 8] ^= 1 << (k % 8);
            }
            peer.faults.key = Some(key);
            failed(&mut Run::new(peer), DHKEY_CHECK_FAILED, &format!("round {round}"));
        }
    }
}
