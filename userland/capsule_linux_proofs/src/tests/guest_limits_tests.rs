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

//! What a guest can make this capsule hold: tables of pipes, counters,
//! timers and signalfd masks that grow only as far as the guest holds
//! descriptors, control data that is walked to its end and no further
//! whatever lengths it claims, a resolver that takes and keeps what a
//! datagram socket would, and a display stream that a malformed header
//! cannot stall.

use super::random::Regs;
use crate::linux::abi::errno::EMSGSIZE;
use crate::linux::guest::links_room::{room, MAX_LINKS};
use crate::linux::guest::slots::{pick, MAX_SLOTS};
use crate::linux::net::dns::limits::{keeps, query_len, MAX_QUERY, MAX_REPLIES};
use crate::linux::net::dns::{name, reply};
use crate::linux::unix::cmsg::{rights, MAX_FDS};
use crate::wayland_object::{Object, Objects, MAX_OBJECTS};
use crate::wire::{malformed, next};

#[test]
fn a_slot_nothing_names_is_taken_before_the_table_grows() {
    assert_eq!(pick(0, |_| false), Some(0));
    assert_eq!(pick(3, |_| false), Some(3));
    assert_eq!(pick(3, |i| i == 1), Some(1));
    assert_eq!(pick(3, |i| i >= 1), Some(1), "the lowest free slot");
    assert_eq!(pick(MAX_SLOTS, |_| false), None, "a full table is ENFILE");
    assert_eq!(pick(MAX_SLOTS + 9, |i| i == MAX_SLOTS + 1), None, "nothing past the ceiling");
}

/// A guest that makes and closes a pipe, or is refused one, a hundred
/// thousand times: the table stays the size of what it holds at once.
#[test]
fn making_and_closing_forever_never_grows_the_table() {
    let mut named: Vec<bool> = Vec::new();
    for round in 0..100_000 {
        let Some(slot) = pick(named.len(), |i| !named[i]) else {
            panic!("refused at round {round}");
        };
        match named.get_mut(slot) {
            Some(n) => *n = true,
            None => named.push(true),
        }
        // Every third one is kept open a while; the rest close at once.
        if round % 3 != 0 || round % 30 == 0 {
            named[slot] = false;
        }
        if round % 30 == 0 {
            named.iter_mut().for_each(|n| *n = false);
        }
    }
    assert!(named.len() <= 12, "the table grew to {}", named.len());
}

#[test]
fn symlink_stops_at_the_ceiling_as_a_full_filesystem_does() {
    assert!(room(0) && room(MAX_LINKS - 1));
    assert!(!room(MAX_LINKS), "one more is ENOSPC");
    // A guest making links in a loop stops at the ceiling, however long it runs.
    let mut table = 0usize;
    for _ in 0..(MAX_LINKS * 3) {
        if room(table) {
            table += 1;
        }
    }
    assert_eq!(table, MAX_LINKS);
}

fn block(level: u32, kind: u32, fds: &[u32]) -> Vec<u8> {
    let len = 16 + 4 * fds.len();
    let mut b = Vec::new();
    b.extend_from_slice(&(len as u64).to_le_bytes());
    b.extend_from_slice(&level.to_le_bytes());
    b.extend_from_slice(&kind.to_le_bytes());
    fds.iter().for_each(|fd| b.extend_from_slice(&fd.to_le_bytes()));
    b.resize((len + 7) & !7, 0);
    b
}

#[test]
fn scm_rights_descriptors_are_read_from_every_rights_block_and_nothing_else() {
    let mut raw = block(1, 1, &[3, 4, 5]);
    raw.extend(block(1, 2, &[9, 9])); // SCM_CREDENTIALS-shaped: stepped over
    raw.extend(block(6, 1, &[9])); // another level: stepped over
    raw.extend(block(1, 1, &[7]));
    assert_eq!(rights(&raw), [3, 4, 5, 7]);
    let many: Vec<u32> = (0..300).collect();
    assert_eq!(rights(&block(1, 1, &many)).len(), MAX_FDS, "SCM_MAX_FD");
}

#[test]
fn a_length_that_steps_back_or_past_the_end_ends_the_walk() {
    // A second header whose length, added to where it starts, wraps round to
    // the first: the old walk went back to the top of the block for ever.
    let mut back = block(0, 0, &[]);
    back.extend_from_slice(&(u64::MAX - 15).to_le_bytes());
    back.extend_from_slice(&[0; 8]);
    assert_eq!(rights(&back), [] as [u32; 0]);
    let good = block(1, 1, &[3]);
    let mut raw = good.clone();
    raw.extend_from_slice(&(u64::MAX - 10).to_le_bytes());
    raw.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(rights(&raw), [3]);
    // Shorter than a header, longer than what is left.
    let mut short = good.clone();
    short.extend_from_slice(&8u64.to_le_bytes());
    short.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(rights(&short), [3]);
    let mut long = good;
    long.extend_from_slice(&64u64.to_le_bytes());
    long.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0]);
    assert_eq!(rights(&long), [3]);
}

#[test]
fn a_resolver_takes_one_datagram_and_keeps_a_socket_s_worth() {
    assert_eq!(query_len(MAX_QUERY), Ok(MAX_QUERY as usize));
    assert_eq!(query_len(MAX_QUERY + 1), Err(EMSGSIZE));
    assert_eq!(query_len(u64::MAX), Err(EMSGSIZE), "never sizes a buffer");
    assert!(keeps(MAX_REPLIES - 1) && !keeps(MAX_REPLIES));
}

#[test]
fn a_header_shorter_than_a_header_is_malformed_and_never_waits() {
    let mut buf = vec![1, 0, 0, 0, 0, 0, 4, 0];
    assert!(malformed(&buf) && next(&buf).is_none());
    buf[6] = 8;
    assert!(!malformed(&buf) && next(&buf).is_some());
    buf[6] = 12;
    assert!(!malformed(&buf) && next(&buf).is_none(), "incomplete, not malformed");
    assert!(!malformed(&buf[..7]));
}

#[test]
fn a_display_client_holds_at_most_max_objects_and_may_always_destroy_one() {
    let mut objects = Objects::new();
    // Object 1 is the display; fill the rest.
    for id in 2..=MAX_OBJECTS as u32 {
        assert!(objects.put(id, Object::Callback), "{id}");
    }
    assert!(!objects.put(MAX_OBJECTS as u32 + 1, Object::Buffer), "one more is refused");
    assert!(objects.get(MAX_OBJECTS as u32 + 1).is_none(), "and nothing is recorded");
    assert!(objects.put(7, Object::Buffer), "an id it holds may change what it names");
    assert!(objects.get(7) == Some(Object::Buffer));
    objects.drop_id(7);
    assert!(objects.put(MAX_OBJECTS as u32 + 1, Object::Buffer), "a destroy makes room");
}

/// Random control blocks, queries and display streams: nothing panics, the
/// walk ends, and what comes out lies within what went in.
#[test]
fn random_guest_bytes_never_panic_or_run_on() {
    let mut r = Regs::new(0x6775_6573_745f_6c69);
    for _ in 0..20_000 {
        let len = r.small(1100) as usize;
        let mut raw: Vec<u8> = (0..len).map(|_| r.any() as u8).collect();
        // Plant plausible headers so the walk goes some way in.
        for at in (0..len.saturating_sub(16)).step_by(8 + 8 * r.small(4) as usize) {
            if r.small(2) == 0 {
                raw[at..at + 8].copy_from_slice(&(16 + 4 * r.small(8)).to_le_bytes());
                raw[at + 8..at + 16].copy_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
            }
        }
        let fds = rights(&raw);
        assert!(fds.len() <= MAX_FDS && fds.len() * 4 <= raw.len());
        if let Some((host, end)) = name::read(&raw) {
            assert!(host.len() <= 255 && end <= raw.len());
            if let Some(question) = raw.get(..end + 4) {
                assert!(reply::build(question, Some([1, 2, 3, 4])).len() <= question.len() + 16);
            }
        }
        let _ = malformed(&raw);
        if let Some((m, used)) = next(&raw) {
            assert!(used >= 8 && used <= raw.len() && m.args.len() == used - 8);
        }
    }
}
