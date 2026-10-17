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

//! E-67: command credits from a hostile controller (Vol 4, Part E, 4.4).
//! Num_HCI_Command_Packets is the controller's to set and it may grant it
//! unasked, so the host honours it but clamps it to its own bound: a
//! controller granting the most it can never pushes the count past that
//! bound nor overflows it, and credits alone never finish a command.

use crate::sim::config::{Config, ADDR};
use crate::sim::events::complete;
use crate::sim::stack::{stack, stack_with_budget};
use nonos_bt_hci::host::COMMAND_CREDITS_MAX;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];

fn draw(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn granting_the_most_it_can_never_pushes_credits_past_the_bound() {
    for mut r in SEEDS {
        let mut s = stack(Config { silent: true, ..Config::default() });
        for _ in 0..300 {
            let p = s.transport().pipes();
            // A count-only grant for another command may come first, then
            // the answer; both carry a hostile count, up to 255.
            if draw(&mut r).is_multiple_of(2) {
                p.send_event(&complete(draw(&mut r) as u8, 0x0000, 0, &[]));
            }
            let grant = 1 + (draw(&mut r) % 255) as u8;
            p.send_event(&complete(grant, op::READ_BD_ADDR.0, 0, &ADDR));
            assert!(s.command(op::READ_BD_ADDR, &[]).is_ok(), "a sound grant stalled");
            assert!(s.credits() <= COMMAND_CREDITS_MAX, "credits past the bound");
        }
    }
}

#[test]
fn credits_alone_never_complete_a_command() {
    let mut s = stack_with_budget(Config { silent: true, ..Config::default() }, 8);
    let p = s.transport().pipes();
    // A flood of maximal grants, none for the command the host waits on.
    (0..8).for_each(|_| p.send_event(&complete(255, 0x0000, 0, &[])));
    let waited = op::READ_BD_ADDR;
    assert_eq!(s.command(waited, &[]), Err(HciError::Timeout(waited)));
    assert!(s.credits() <= COMMAND_CREDITS_MAX, "the flood raised the count");
    // The matching positive: the answer itself completes the command.
    s.transport().pipes().send_event(&complete(255, waited.0, 0, &ADDR));
    assert_eq!(s.command(waited, &[]).unwrap(), ADDR);
}
