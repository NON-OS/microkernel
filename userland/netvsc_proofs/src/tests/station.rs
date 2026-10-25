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

//! The the maintainers' ruling on the station address: the drawn address first; the
//! host-assigned one when the host refuses it, or when nothing answers it
//! within the bounded wait; never silently, never before the wait.

use super::util::{netvsp, up, NetLink};
use crate::constants::timing::{PROBE_EVERY_MS, PROBE_TRIES};
use crate::probe::Verdict;
use crate::station::{address, choose};

/// A drawn address netvsp parses: its string starts with a digit.
const DRAWN: [u8; 6] = [0x02, 0x5A, 0x11, 0x22, 0x33, 0x44];
/// One netvsp refuses: its string starts with a letter.
const DRAWN_LETTER: [u8; 6] = [0xA6, 0x5A, 0x11, 0x22, 0x33, 0x44];
const XID: u32 = 0x1234_5678;

fn settled(spoofing: bool, dhcp: bool, drawn: [u8; 6]) -> (NetLink, Verdict, u64) {
    let (mut l, _) = up();
    l.host.netvsp.wire.spoofing = spoofing;
    l.host.netvsp.wire.dhcp = dhcp;
    let t0 = l.host.vmbus.now;
    let v = choose(&mut l, &drawn, XID).unwrap();
    let waited = l.host.vmbus.now - t0;
    (l, v, waited)
}

#[test]
fn a_host_that_carries_the_drawn_address_keeps_it() {
    let (l, v, waited) = settled(true, true, DRAWN);
    assert_eq!(v, Verdict::Drawn);
    assert_eq!(netvsp(&l).taken, Some(DRAWN), "the host was asked to take it");
    assert_eq!(netvsp(&l).wire.offers, 1, "one DISCOVER was enough");
    assert!(waited < PROBE_EVERY_MS, "no wait beyond the answer: {waited}");
    assert_eq!(address(v, DRAWN, netvsp(&l).mac), DRAWN);
}

#[test]
fn mac_spoofing_off_falls_back_only_after_the_whole_wait() {
    let (l, v, waited) = settled(false, true, DRAWN);
    assert_eq!(v, Verdict::Unanswered);
    // Every DISCOVER was sent from the drawn address and the switch dropped
    // each; nothing came back.
    let tx = &netvsp(&l).tx;
    assert_eq!(tx.len(), PROBE_TRIES as usize);
    assert!(tx.iter().all(|f| f[6..12] == DRAWN));
    assert!(netvsp(&l).wire.out.is_empty(), "the switch carried none");
    assert_eq!(netvsp(&l).wire.offers, 0);
    let bound = u64::from(PROBE_TRIES) * PROBE_EVERY_MS;
    assert!(waited >= bound, "fell back before the bounded wait: {waited} < {bound}");
    assert!(waited < bound + PROBE_EVERY_MS, "waited past the bound: {waited}");
    assert_eq!(address(v, DRAWN, netvsp(&l).mac), netvsp(&l).mac);
}

#[test]
fn a_host_that_refuses_the_drawn_address_falls_back_at_once() {
    // netvsp:3964-3971: a string not starting with a digit is INVALID_DATA.
    let (l, v, waited) = settled(true, true, DRAWN_LETTER);
    assert_eq!(v, Verdict::Refused);
    assert!(netvsp(&l).tx.is_empty(), "no DISCOVER from a refused address");
    assert!(waited < PROBE_EVERY_MS);
    assert_eq!(address(v, DRAWN_LETTER, netvsp(&l).mac), netvsp(&l).mac);
}

#[test]
fn a_network_with_no_dhcp_server_falls_back_too() {
    // The ruling's cost, stated: a quiet network looks like a switch that
    // drops, and the host-assigned address is used.
    let (_, v, _) = settled(true, false, DRAWN);
    assert_eq!(v, Verdict::Unanswered);
}

#[test]
fn an_answer_to_another_transaction_or_address_does_not_count() {
    use crate::probe::{answers, discover};
    let mine = discover(&DRAWN, XID);
    assert!(!answers(&mine, &DRAWN, XID), "our own broadcast DISCOVER is no answer");
    let mut wire = crate::model::wire::Wire::new();
    let offer = wire.answer(&discover(&DRAWN, XID)).remove(0);
    assert!(answers(&offer, &DRAWN, XID));
    assert!(!answers(&offer, &DRAWN, XID + 1), "another transaction");
    assert!(!answers(&offer, &DRAWN_LETTER, XID), "another address");
    let mut unicast = vec![0u8; 60];
    unicast[..6].copy_from_slice(&DRAWN);
    assert!(answers(&unicast, &DRAWN, XID), "any frame sent to the drawn address");
}
