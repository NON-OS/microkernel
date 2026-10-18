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

//! EAPOL once the keys are in. A plaintext EAPOL frame is handshake traffic
//! and nothing else: it goes to the supplicant, which answers a genuine
//! message and drops the rest, and never to the stack. One sent to a group
//! address is no handshake of this link's and is refused unanswered. A
//! group key of all zeros is never installed.

use super::air::{data, four_way, group, seal};
use super::watch::{delivered, put};
use crate::ap_sim::AA;
use crate::soft::data_frames::{eapol_from, eapol_of};
use crate::soft::join::{joined, sta_of};
use crate::soft::rig::Station;

/// EAPOL frames the station has sent.
fn answers(st: &Station) -> usize {
    st.radio().unwrap().sent.iter().filter(|f| eapol_of(f).is_some()).count()
}

#[test]
fn eapol_is_handshake_traffic_never_the_stacks_and_never_group_addressed() {
    let mut st = joined();
    let (start, said) = (delivered(&st), answers(&st));
    put(&mut st, |s| four_way(s, true, 3));
    assert_eq!(answers(&st), said + 1, "message 3 again: message 4 again");
    let junk = [1, 0, 0, 4, 1, 2, 3, 4];
    put(&mut st, |s| eapol_from(AA, sta_of(s), &junk));
    put(&mut st, |s| seal(s, &eapol_from(AA, sta_of(s), &junk), 1));
    let mut to_all = four_way(&st, true, 4);
    to_all[4..10].copy_from_slice(&[0xff; 6]);
    put(&mut st, |_| to_all);
    assert_eq!(answers(&st), said + 1, "nothing else answered");
    assert_eq!(delivered(&st), start, "and nothing to the stack");
    put(&mut st, |s| four_way(s, true, 4));
    assert_eq!(answers(&st), said + 2, "the group-addressed one moved no counter");
}

#[test]
fn a_group_key_of_zeros_is_never_installed() {
    let mut st = joined();
    let (start, said) = (delivered(&st), answers(&st));
    put(&mut st, |s| group(s, 3, &[0; 16], 2));
    assert_eq!(answers(&st), said, "not answered");
    put(&mut st, |s| data(s, 1, Some((&[0; 16], 2))));
    assert_eq!(delivered(&st), start, "nothing taken under it");
}
