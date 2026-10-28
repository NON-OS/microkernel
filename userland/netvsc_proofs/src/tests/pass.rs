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

//! One service pass takes a bounded number of packets, and a channel the
//! host broke is left as a device present that failed, not an absent one.

use nonos_vmbus::Host;

use super::util::up;
use crate::constants::exit::CHANNEL_BROKEN;
use crate::constants::layout::PACKETS_PER_PASS;

#[test]
fn one_pass_takes_at_most_its_bound_and_the_next_takes_the_rest() {
    let (mut l, _) = up();
    // The first PACKETS_PER_PASS say up, the rest down: one per packet.
    for i in 0..PACKETS_PER_PASS + 30 {
        l.host.netvsp.indicate_link(i < PACKETS_PER_PASS);
    }
    l.host.idle();
    l.service().unwrap();
    assert_eq!(l.link, Some(true), "the pass stopped at its bound");
    l.service().unwrap();
    assert_eq!(l.link, Some(false), "the next pass took the rest");
}

#[test]
fn a_broken_channel_leaves_with_the_policy_s_give_up_code() {
    let policy = include_str!("../../../libc/src/bringup/policy.rs");
    assert!(policy.contains("pub const EXIT_GAVE_UP: i32 = 6;"));
    assert!(policy.contains("pub const EXIT_ABSENT: i32 = 2;"));
    assert_eq!(CHANNEL_BROKEN, 6, "EXIT_GAVE_UP");
}
