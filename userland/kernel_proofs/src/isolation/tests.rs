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
//! A against B, by random schedules. The oracle below knows only what B
//! consented to and when it took it back; it reads none of the rules.

use super::model::{handle, Machine, SLOTS};

const A: u32 = 10;
const B: u32 = 20;
const C: u32 = 30;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// What B allowed A, per handle: A's incarnation at the time, and write.
#[derive(Default)]
struct Consent {
    given: Vec<(u64, u32, bool)>,
}

impl Consent {
    fn give(&mut self, h: u64, inc: u32, write: bool) {
        match self.given.iter_mut().find(|g| g.0 == h && g.1 == inc) {
            Some(g) => g.2 |= write,
            None => self.given.push((h, inc, write)),
        }
    }
    fn take_handle(&mut self, h: u64) {
        self.given.retain(|g| g.0 != h);
    }
    fn of(&self, h: u64, inc: u32) -> Option<bool> {
        self.given.iter().find(|g| g.0 == h && g.1 == inc).map(|g| g.2)
    }
}

fn run(seed: u64, steps: usize, b_ever_shares_with_a: bool) -> (usize, usize) {
    let mut m = Machine::new();
    let mut rng = Rng(seed);
    let mut consent = Consent::default();
    let mut a_inc = 0u32;
    let mut b_live: Vec<u64> = Vec::new();
    let mut b_closing = false;
    let mut views: Vec<(u64, u64)> = Vec::new();
    let (mut granted, mut refused) = (0, 0);
    for _ in 0..steps {
        match rng.next() % 9 {
            0 | 1 if !b_closing => {
                if let Some(h) = m.create(B) {
                    b_live.push(h);
                }
            }
            2 if !b_live.is_empty() => {
                let h = b_live[rng.next() as usize % b_live.len()];
                let write = rng.next() & 1 == 1;
                let to = if b_ever_shares_with_a && rng.next() & 1 == 0 { A } else { C };
                if m.share(B, h, to, write) && to == A {
                    consent.give(h, a_inc, write);
                }
            }
            3 if !b_live.is_empty() => {
                let h = b_live.swap_remove(rng.next() as usize % b_live.len());
                m.release(B, h);
                consent.take_handle(h);
            }
            4 if rng.next().is_multiple_of(16) => {
                // B exits in two steps; between them it is closing.
                m.exit_begin(B);
                b_closing = true;
                for h in &b_live {
                    consent.take_handle(*h);
                }
            }
            5 if b_closing => {
                m.exit_end(B);
                b_closing = false;
                b_live.clear();
            }
            6 if rng.next().is_multiple_of(32) => {
                // A exits and a new process is given its pid.
                m.exit_begin(A);
                m.exit_end(A);
                a_inc += 1;
            }
            _ => {
                // A tries a handle: one of B's, or a guess at any slot and epoch.
                let h = if !b_live.is_empty() && rng.next() & 1 == 0 {
                    b_live[rng.next() as usize % b_live.len()]
                } else {
                    handle(rng.next() as usize % SLOTS, (rng.next() % 64) as u32)
                };
                let pages = rng.next() % 4 + 1;
                let b_owns = b_live.contains(&h);
                match m.attach(A, h, pages) {
                    Some((base, write)) => {
                        assert!(b_owns, "A mapped a surface B does not hold: {h:#x}");
                        let allowed = consent.of(h, a_inc);
                        assert!(allowed.is_some(), "A mapped B's {h:#x} without B naming it");
                        assert!(!b_closing, "A mapped B's {h:#x} while B was exiting");
                        assert!(!write || allowed == Some(true), "A writes B's {h:#x}, named read-only");
                        for &(v, n) in &views {
                            assert!(base + pages * 4096 + 4096 <= v || v + n * 4096 + 4096 <= base, "a view with no guard");
                        }
                        views.push((base, pages));
                        granted += 1;
                    }
                    None => {
                        if b_owns && !b_closing {
                            assert!(consent.of(h, a_inc).is_none(), "B named A for {h:#x}, yet A was refused");
                        }
                        refused += 1;
                    }
                }
            }
        }
    }
    (granted, refused)
}

#[test]
fn a_maps_only_what_b_named_it_for_and_only_while_it_stands() {
    let mut granted = 0;
    for seed in 1..=64 {
        granted += run(seed, 5_000, true).0;
    }
    assert!(granted > 1_000, "the schedules exercised too few attaches: {granted}");
}

#[test]
fn a_maps_nothing_of_b_when_b_never_names_it() {
    for seed in 1..=64 {
        let (granted, refused) = run(seed, 5_000, false);
        assert_eq!(granted, 0);
        assert!(refused > 0);
    }
}

#[test]
fn a_new_process_on_as_old_pid_inherits_no_share() {
    let mut m = Machine::new();
    let h = m.create(B).expect("slot");
    assert!(m.share(B, h, A, true));
    assert!(m.attach(A, h, 1).is_some());
    m.exit_begin(A);
    m.exit_end(A);
    // The slot still names pid A; the right went with A's exit.
    assert!(m.attach(A, h, 1).is_none());
}

#[test]
fn a_stale_handle_to_a_reused_slot_reaches_nothing() {
    let mut m = Machine::new();
    let old = m.create(B).expect("slot");
    assert!(m.share(B, old, A, false));
    m.release(B, old);
    let new = m.create(B).expect("the same slot, a new epoch");
    assert_ne!(old, new);
    assert!(m.share(B, new, C, false));
    assert!(m.attach(A, old, 1).is_none());
    assert!(m.attach(A, new, 1).is_none());
}

#[test]
fn a_share_with_a_flag_no_one_defined_names_no_one() {
    let mut m = Machine::new();
    let h = m.create(B).expect("slot");
    assert!(!m.share_flags(B, h, A, 2));
    assert!(m.attach(A, h, 1).is_none());
}

#[test]
fn a_handle_and_a_sid_each_travel_in_one_register() {
    // Both cross the syscall boundary as one u64 argument; a handle wider
    // than that would be cut, and its epoch with it.
    use super::handle::{decode_handle, encode_handle, SurfaceHandle, SurfaceSid};
    assert_eq!(core::mem::size_of::<SurfaceHandle>(), 8);
    assert_eq!(core::mem::size_of::<SurfaceSid>(), 8);
    assert_eq!(decode_handle(encode_handle(u32::MAX, u32::MAX)), (u32::MAX, u32::MAX));
}
