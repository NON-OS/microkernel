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

//! One client's share of the socket table, and the sockets of clients that
//! ended without closing them.

use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};

use crate::bounds::{PER_PID_MAX, TABLE_CAP};
use crate::sockets::{Kind, SocketKey, SOCKETS};

static SERIAL: Mutex<()> = Mutex::new(());

/// The one table, emptied, for a proof that holds it for its whole run.
fn fresh() -> MutexGuard<'static, ()> {
    let g = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    SOCKETS.take_dead(|_| false);
    g
}

fn fill(pid: u32, n: usize) -> Vec<SocketKey> {
    (0..n).map(|_| SOCKETS.open(pid, Kind::Mixnet).expect("a socket within the bounds")).collect()
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_the_whole_table() {
    assert!(PER_PID_MAX < TABLE_CAP);
}

/*
 * One client could open every slot, and every other socket on the machine
 * was then refused until it closed them.
 */
#[test]
fn a_client_stops_at_its_share_and_others_still_open() {
    let _g = fresh();
    fill(10, PER_PID_MAX);
    assert!(SOCKETS.open(10, Kind::Stream).is_none(), "past its share");
    assert!(SOCKETS.open(11, Kind::Stream).is_some(), "another client still opens");
}

#[test]
fn a_closed_socket_gives_its_share_back() {
    let _g = fresh();
    let keys = fill(12, PER_PID_MAX);
    assert!(SOCKETS.open(12, Kind::Stream).is_none());
    assert!(SOCKETS.close(keys[0]));
    assert!(SOCKETS.open(12, Kind::Stream).is_some());
}

/*
 * A client that ended without closing kept its sockets for good: a client
 * crashing in a loop filled the table and every later socket was refused.
 */
#[test]
fn the_sockets_of_ended_clients_are_taken_out_and_their_slots_reused() {
    let _g = fresh();
    fill(20, PER_PID_MAX);
    fill(21, TABLE_CAP - PER_PID_MAX);
    assert!(SOCKETS.open(22, Kind::Stream).is_none(), "the table is full");
    let gone = SOCKETS.take_dead(|pid| pid != 20 && pid != 21);
    assert_eq!(gone.len(), TABLE_CAP);
    assert!(gone.iter().all(|s| s.key.pid == 20 || s.key.pid == 21));
    assert!(SOCKETS.open(22, Kind::Stream).is_some(), "the slots are free again");
}

#[test]
fn a_living_client_keeps_every_socket() {
    let _g = fresh();
    let mine = fill(30, 5);
    fill(31, 7);
    let gone = SOCKETS.take_dead(|pid| pid == 30);
    assert_eq!(gone.len(), 7);
    for k in mine {
        assert!(SOCKETS.with(k, |s| s.key.pid) == Some(30), "a living client's socket stays");
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/*
 * Random clients opening, closing and ending: no client ever holds past its
 * share, the table never past its size, a key is never handed out twice,
 * and after a sweep no ended client holds anything.
 */
#[test]
fn clients_opening_closing_and_ending_keep_every_bound() {
    let _g = fresh();
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    let mut ended: HashSet<u32> = HashSet::new();
    let mut open: Vec<SocketKey> = Vec::new();
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    for _ in 0..20_000 {
        let r = xorshift(&mut s);
        let pid = 100 + (r % 6) as u32;
        match (r >> 8) % 10 {
            0..=5 if !ended.contains(&pid) => {
                if let Some(k) = SOCKETS.open(pid, Kind::Stream) {
                    assert!(seen.insert((k.pid, k.handle)), "a key handed out twice");
                    open.push(k);
                }
            }
            6..=7 if !open.is_empty() => {
                let k = open.swap_remove((r >> 16) as usize % open.len());
                assert!(SOCKETS.close(k));
            }
            8 => {
                ended.insert(pid);
            }
            _ => {
                let gone = SOCKETS.take_dead(|p| !ended.contains(&p));
                assert!(gone.iter().all(|g| ended.contains(&g.key.pid)));
                open.retain(|k| !ended.contains(&k.pid));
                ended.clear();
            }
        }
        assert!(open.len() <= TABLE_CAP);
        for p in 100..106 {
            assert!(open.iter().filter(|k| k.pid == p).count() <= PER_PID_MAX);
        }
    }
}
