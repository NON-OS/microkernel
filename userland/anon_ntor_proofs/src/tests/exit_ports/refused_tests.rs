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

//! An exit that refused a port is remembered for that port, in a bounded
//! list, and no draw for the port picks it again.

use crate::path::refused::{Refused, REFUSED_MAX};
use crate::path::through;

use super::draw_tests::{dice, exits};
use crate::tests::path_pool::{flat, pool, relay};

#[test]
fn a_refusal_is_for_the_exit_and_the_port_both() {
    let mut r = Refused::default();
    r.note([2; 20], 18081);
    assert!(r.holds(&[2; 20], 18081));
    assert!(!r.holds(&[2; 20], 18089), "another port of the same exit");
    assert!(!r.holds(&[4; 20], 18081), "another exit for the same port");
}

#[test]
fn the_list_is_bounded_and_forgets_the_oldest_first() {
    let mut r = Refused::default();
    for port in 1..=(REFUSED_MAX as u16 + 10) {
        r.note([7; 20], port);
        r.note([7; 20], port);
    }
    assert!(!r.holds(&[7; 20], 10), "the oldest went to make room");
    assert!(r.holds(&[7; 20], 11), "the oldest kept is just past them");
    let kept = (1..=u16::MAX).filter(|&p| r.holds(&[7; 20], p)).count();
    assert_eq!(kept, REFUSED_MAX, "a refusal noted twice is held once");
}

#[test]
fn an_exit_that_refused_a_port_is_never_drawn_for_it_again() {
    let (relays, mut exits) = (pool(), exits());
    exits.refused.note([2; 20], 18081);
    exits.refused.note([6; 20], 18081);
    let exit_2 = relay([10, 2, 0, 1], 2);
    assert!(!exits.takes(&exit_2, 18081) && exits.takes(&exit_2, 443));
    let mut drawn = [false; 13];
    for seed in 0..2_000u64 {
        let path = through(&relays[0], &relays, &flat(), |e| exits.takes(e, 18081), dice(seed));
        drawn[path.unwrap()[2].rsa_identity[0] as usize] = true;
    }
    assert!(!drawn[2] && !drawn[6], "the refusing exits are passed over");
    assert!(drawn[4] && drawn[8] && drawn[10] && drawn[12], "the others still are drawn");
    let web = (0..2_000u64)
        .filter_map(|s| through(&relays[0], &relays, &flat(), |e| exits.takes(e, 443), dice(s)))
        .any(|path| path[2].rsa_identity == [2; 20]);
    assert!(web, "for the web ports it is drawn as before");
}
