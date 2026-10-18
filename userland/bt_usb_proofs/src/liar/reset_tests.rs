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

//! A controller that resets itself mid-link. With a word (Hardware Error,
//! or Command Complete for a Reset no one sent) the link and the
//! controller's run end at once; without one, the link has gone quiet and
//! the first check after 10 s finds the controller no longer has it.
//! Either way the key held down goes up.

use super::{A_DOWN, KEYS};
use crate::air::{Act, Lies};
use crate::fixture::{air, keyboard, link};
use crate::hci_sim::events::complete;
use nonos_bt_hci::HciError;
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::hci::opcode as op;

fn reset_while_a_key_is_down(lie: Act) -> (Result<(), HciError>, Vec<Input>, u64) {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.extend([Act::Notify(KEYS, A_DOWN.to_vec()), lie]);
    let r = link(&mut host, &ours, &mut env).map(|_| ());
    (r, env.input, env.now)
}

#[test]
fn a_reset_said_out_loud_ends_the_link_and_lets_the_key_go() {
    for lie in [vec![0x10, 1, 0x00], complete(1, op::RESET.0, 0, &[])] {
        let (r, input, ..) = reset_while_a_key_is_down(Act::Event(lie));
        assert_eq!(r, Err(HciError::Reset));
        assert_eq!(input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    }
}

#[test]
fn a_silent_reset_is_found_by_the_first_check_of_a_quiet_link() {
    let forget = Act::Lie(|l: &mut Lies| l.forgot = true);
    let (r, input, now) = reset_while_a_key_is_down(forget);
    assert_eq!(r, Err(HciError::Reset));
    assert_eq!(input, [Input::KeyDown(0x04), Input::KeyUp(0x04)]);
    assert!((10_000..25_000).contains(&now), "found at {now} ms");
}
