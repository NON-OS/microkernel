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

//! The classifier over seeded frames: it completes only the waited command,
//! an unknown event completes nothing, and a lying length is refused — never
//! a panic.

use super::kit::{cc, cs, frame, unknown, Rng, CC, CS, SEEDS};
use nonos_bt_hci::host::answer;
use nonos_bt_wire::hci::Opcode;

const WAIT: u16 = 0x0c03; // the opcode the host is waiting on

#[test]
fn an_event_completes_only_the_command_waited_for() {
    for seed in SEEDS {
        let mut rng = Rng(seed);
        for _ in 0..3000 {
            let opcode = if rng.below(2) == 0 { WAIT } else { rng.next() as u16 };
            let (code, params) = match rng.below(3) {
                0 => (CC, cc(rng.byte(), opcode, &[])),
                1 => (CS, cs(rng.byte(), rng.byte(), opcode)),
                // A genuine unknown: a code that is neither answer event.
                _ => (unknown(rng.byte()), (0..rng.below(8)).map(|_| rng.byte()).collect()),
            };
            let f = frame(code, params.len() as u8, &params);
            match answer(&f, Some(Opcode(WAIT))).expect("well formed") {
                // An unknown event answers no command at all.
                None => assert!(code != CC && code != CS, "a sound answer event said nothing"),
                Some((_, a)) => {
                    let completes = (code == CC || code == CS) && opcode == WAIT;
                    assert_eq!(a.is_some(), completes, "completed the wrong command");
                }
            }
        }
    }
}

#[test]
fn a_lying_length_is_refused_and_nothing_panics() {
    for seed in SEEDS {
        let mut rng = Rng(seed);
        for _ in 0..3000 {
            let ret: Vec<u8> = (0..rng.below(8)).map(|_| rng.byte()).collect();
            let params = cc(rng.byte(), 0x0000, &ret); // opcode 0, never the waited WAIT
            let sound = frame(CC, params.len() as u8, &params);
            // Sound: parses, but answers no waited command.
            assert!(matches!(answer(&sound, Some(Opcode(WAIT))), Ok(Some((_, None)))));
            // A stated length that is not the real one, refused.
            let mut lie = sound.clone();
            let d = rng.byte();
            lie[1] = if d == 0 { lie[1].wrapping_add(1) } else { lie[1].wrapping_add(d) };
            assert!(answer(&lie, Some(Opcode(WAIT))).is_err(), "a length lie opened");
            // Cut short of the stated length, refused.
            let cut = &sound[..rng.below(sound.len())];
            assert!(answer(cut, Some(Opcode(WAIT))).is_err(), "a short frame opened");
        }
    }
}
