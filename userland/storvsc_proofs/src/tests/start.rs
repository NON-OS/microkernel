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

//! The channel and the protocol's start: one GPADL over the rings alone,
//! the open, then BEGIN, the newest version the host takes, the properties
//! and END, each in band on the start's id, and each refusal or silence a
//! failed start.

use super::util::{opened, ops, seen, started, RELID};
use crate::constants::layout::DATA_BYTES;
use crate::constants::timing::INIT_MS;
use crate::constants::vstor::INIT_ID;
use crate::model::{Server, BASE_PFN};
use crate::start;

#[test]
fn the_channel_opens_on_the_rings_alone() {
    let h = opened(Server::new(vec![]));
    let m = &h.host.vmbus;
    let pfns: Vec<u64> = (0..64).map(|i| BASE_PFN + i).collect();
    assert_eq!(m.gpadls, [(0x5343_0000 | RELID, pfns)], "the data buffer is never in the GPADL");
    assert_eq!(m.open, Some((RELID, 0x5343_0000 | RELID, 32)), "the host's ring 32 pages in");
    assert_eq!(&m.posted.last().unwrap()[28..52], &[0u8; 24], "zeroed channel properties");
}

#[test]
fn the_start_asks_win10_first_and_takes_win8_1_from_storvsp() {
    // OpenVMM's storvsp takes WIN6 to WIN8.1 (storvsp:235-251), so WIN10 is
    // refused with REVISION_MISMATCH and WIN8.1 agreed.
    let mut h = opened(Server::new(vec![]));
    assert_eq!(start::run(&mut h), Ok(0x0600));
    assert_eq!(ops(&h), [7, 9, 9, 10, 8]);
    assert!(seen(&h).iter().all(|s| s.ty == 6 && s.txid == INIT_ID && s.range.is_none()));
    assert_eq!(&seen(&h)[1].packet[12..16], &[0x02, 0x06, 0, 0]);
    assert_eq!(&seen(&h)[2].packet[12..16], &[0x00, 0x06, 0, 0]);
    assert_eq!(h.limit, DATA_BYTES, "storvsp takes 256 KiB; the buffer is 64");
}

#[test]
fn a_host_that_takes_win10_is_spoken_to_in_it() {
    let mut s = Server::new(vec![]);
    s.versions.push(0x0602);
    let mut h = opened(s);
    assert_eq!(start::run(&mut h), Ok(0x0602));
    assert_eq!(ops(&h), [7, 9, 10, 8]);
}

#[test]
fn a_host_older_still_is_left() {
    let mut s = Server::new(vec![]);
    s.versions = vec![0x0501, 0x0402];
    let mut h = opened(s);
    assert_eq!(start::run(&mut h), Err("no protocol version in common"));
    assert_eq!(ops(&h), [7, 9, 9], "no END after a failed start");
}

#[test]
fn each_refusal_fails_the_start_where_it_comes() {
    for (op, why, sent) in [
        (7, "begin refused", vec![7]),
        (9, "no protocol version in common", vec![7, 9, 9]),
        (10, "properties refused", vec![7, 9, 9, 10]),
        (8, "end refused", vec![7, 9, 9, 10, 8]),
    ] {
        let mut s = Server::new(vec![]);
        s.refuse = vec![op];
        let mut h = opened(s);
        assert_eq!(start::run(&mut h), Err(why));
        assert_eq!(ops(&h), sent);
    }
}

#[test]
fn a_smaller_transfer_limit_is_kept_in_whole_pages() {
    let mut s = Server::new(vec![]);
    s.max_transfer = 0x2A00;
    assert_eq!(started(s).limit, 0x2000);
    let mut s = Server::new(vec![]);
    s.max_transfer = 0x1000;
    assert_eq!(started(s).limit, 0x1000, "one page is enough");
    let mut s = Server::new(vec![]);
    s.max_transfer = 0xFFF;
    assert_eq!(start::run(&mut opened(s)), Err("transfers too small"));
}

#[test]
fn a_silent_host_fails_the_start_in_bounded_time() {
    let mut s = Server::new(vec![]);
    s.silent = 1;
    let mut h = opened(s);
    let t0 = h.host.vmbus.now;
    assert_eq!(start::run(&mut h), Err("host silent"));
    assert!(h.host.vmbus.now - t0 >= INIT_MS);
    assert_eq!(ops(&h), [7]);
}

#[test]
fn a_short_answer_fails_the_start() {
    let mut s = Server::new(vec![]);
    s.short_next = true;
    let mut h = opened(s);
    assert_eq!(start::run(&mut h), Err("answer short"));
}
