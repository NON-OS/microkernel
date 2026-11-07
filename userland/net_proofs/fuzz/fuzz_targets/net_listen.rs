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

#![no_main]

extern crate alloc;

use libfuzzer_sys::fuzz_target;
use nonos_policy_proto::route;

#[path = "../../../capsule_net_core/src/listeners/gate.rs"]
mod gate;
#[path = "../../../capsule_net_core/src/listeners/table.rs"]
mod table;
use table::{Listeners, Refusal, LISTENER_BASE};

const N: usize = 4;
const B: usize = 4;

/// What the table should hold: per slot, its owner pid, the port, the listening
/// socket and the queue oldest first.
type Model = [Option<(u32, u16, u16, Vec<u16>)>; N];

fn handle_of(b: u8) -> u32 {
    match b % 8 {
        0 => 0,
        1 => 1,
        2 => LISTENER_BASE,
        3 => LISTENER_BASE + N as u32 + 1,
        _ => LISTENER_BASE + u32::from(b % 8 - 3),
    }
}

fn slot(handle: u32) -> Option<usize> {
    let at = handle.checked_sub(LISTENER_BASE + 1)? as usize;
    (at < N).then_some(at)
}

fn check(t: &mut Listeners<u16, N, B>, m: &Model, ready: impl Fn(u16) -> bool) {
    let mut seen = Vec::new();
    let mut count = 0;
    t.for_each_mut(|_| count += 1);
    assert_eq!(count, m.iter().flatten().count());
    for at in 0..N {
        let handle = LISTENER_BASE + at as u32 + 1;
        assert!(Listeners::<u16, N, B>::is_listener(handle));
        for owner in 1..=3 {
            let got = t.get_mut(handle, owner).map(|l| (l.port, l.all_sockets()));
            let want = m[at].as_ref().filter(|e| e.0 == owner).map(|e| {
                let mut all = vec![e.2];
                all.extend(&e.3);
                (e.1, all)
            });
            assert_eq!(got, want, "slot {at} owner {owner}");
            let any = t.get_mut(handle, owner).map(|l| l.any_ready(&ready));
            let want =
                m[at].as_ref().filter(|e| e.0 == owner).map(|e| e.3.iter().any(|&h| ready(h)));
            assert_eq!(any, want, "slot {at} owner {owner}");
        }
        if let Some(e) = &m[at] {
            assert!(e.3.len() <= B);
            seen.push(e.2);
            seen.extend(&e.3);
        }
    }
    let n = seen.len();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), n, "a socket held twice");
    assert_eq!(t.any(), m.iter().any(|s| s.is_some()));
    for h in [0, 1, LISTENER_BASE, LISTENER_BASE + N as u32 + 1, u32::MAX] {
        assert!(!Listeners::<u16, N, B>::is_listener(h), "{h:#x}");
    }
}

fn gate_holds(b: &[u8]) {
    let sender = u32::from(b[0] % 4);
    let sockets = (b[1] % 5 != 0).then_some(u32::from(b[1] % 4));
    let chosen = (b[2] % 5 != 0).then(|| b[2] % 5 - 1);
    let direct = chosen == Some(route::DIRECT);
    let want = direct && sockets.is_some_and(|p| p != 0 && p == sender);
    assert_eq!(gate::listen_allowed(sender, sockets, chosen), want);
    assert_eq!(gate::listeners_may_stay(chosen), direct);
    assert_eq!(gate::accept_allowed(chosen), direct);
    assert_eq!(gate::passive_may_stay(chosen), direct);
}

fuzz_target!(|data: &[u8]| {
    let mut t = Listeners::<u16, N, B>::new();
    let mut m: Model = Default::default();
    let mut next: u16 = 1;
    for op in data.chunks_exact(3) {
        gate_holds(op);
        let (handle, owner) = (handle_of(op[1]), u32::from(op[2] % 3) + 1);
        let mine = slot(handle).filter(|&at| m[at].as_ref().is_some_and(|e| e.0 == owner));
        match op[0] % 7 {
            0 => {
                let port = u16::from(op[1] % 6);
                let got = t.open(owner, port, next);
                let want = if m.iter().flatten().any(|e| e.1 == port) {
                    Err(Refusal::PortInUse)
                } else if let Some(at) = m.iter().position(|s| s.is_none()) {
                    m[at] = Some((owner, port, next, Vec::new()));
                    Ok(LISTENER_BASE + at as u32 + 1)
                } else {
                    Err(Refusal::Full)
                };
                assert_eq!(got, want);
                next += 1;
            }
            1 => {
                let got = t.close(handle, owner);
                let want = mine.and_then(|at| m[at].take()).map(|e| {
                    let mut all = vec![e.2];
                    all.extend(e.3);
                    all
                });
                assert_eq!(got, want);
            }
            2 => {
                let pushed = t.get_mut(handle, owner).map(|l| l.push(next));
                let want = mine.map(|at| {
                    let q = &mut m[at].as_mut().unwrap().3;
                    let room = q.len() < B;
                    if room {
                        q.push(next);
                    }
                    room
                });
                assert_eq!(pushed, want);
                next += 1;
            }
            3 | 4 => {
                let ready = |h: u16| (h ^ u16::from(op[2])) % 3 == 0;
                let got = t.get_mut(handle, owner).map(|l| l.take(ready));
                let want = mine.map(|at| {
                    let q = &mut m[at].as_mut().unwrap().3;
                    q.iter().position(|&h| ready(h)).map(|i| q.remove(i))
                });
                assert_eq!(got, want);
                if let (Some(Some(h)), 4) = (got, op[0] % 7) {
                    let back = t.get_mut(handle, owner).map(|l| l.restore(h));
                    assert_eq!(back, Some(true), "a taken connection always fits back");
                    m[mine.unwrap()].as_mut().unwrap().3.insert(0, h);
                }
            }
            5 => {
                let dead = u32::from(op[1] % 4);
                let mut got = t.take_ended(|o| o != dead);
                let mut want = Vec::new();
                for s in m.iter_mut() {
                    if s.as_ref().is_some_and(|e| e.0 == dead) {
                        let e = s.take().unwrap();
                        want.push(e.2);
                        want.extend(e.3);
                    }
                }
                got.sort_unstable();
                want.sort_unstable();
                assert_eq!(got, want);
            }
            _ => {
                let keep = |h: u16| h % 2 == u16::from(op[2] % 2);
                let got = t.get_mut(handle, owner).map(|l| l.drop_queued(keep));
                let want = mine.map(|at| {
                    let q = &mut m[at].as_mut().unwrap().3;
                    let (kept, gone) = q.iter().partition(|&&h| keep(h));
                    *q = kept;
                    gone
                });
                assert_eq!(got, want);
            }
        }
        check(&mut t, &m, |h| (h ^ u16::from(op[0])) % 2 == 0);
    }
    if data.len() % 2 == 1 {
        t.forget_all();
        assert!(!t.any());
        return;
    }
    let mut all = t.take_all();
    let mut want: Vec<u16> =
        m.iter().flatten().flat_map(|e| core::iter::once(e.2).chain(e.3.iter().copied())).collect();
    all.sort_unstable();
    want.sort_unstable();
    assert_eq!(all, want);
    assert!(!t.any());
});
