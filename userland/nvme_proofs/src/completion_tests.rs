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

//! The admin and I/O completion waits share one loop. Every completion entry
//! is written by the controller, so the proofs run that loop over scripted
//! rings: an entry is taken only when its phase tag is this pass's and it
//! names the command in flight, any status bit fails the command, the SQ
//! head the controller reports never moves the driver's cursor, every slot
//! read and every doorbell value stays inside the ring, and the wait ends
//! once its deadline passes.

use core::cell::{Cell, RefCell};

use crate::admin::{wait_for_completion, Completion, CqCursor, DEADLINE_CHECK_SPINS};
use crate::error::{NvmeError, NvmeResult};

/// Ring sizes: one and two entries for the wrap edges, the I/O queue's 8 and
/// the admin queue's 64.
pub(crate) const RINGS: [u16; 4] = [1, 2, 8, 64];

pub(crate) fn entry(cid: u16, sq_id: u16, sq_head: u16, status: u16) -> Completion {
    Completion { dw0: 0, dw1: 0, sq_head, sq_id, cid, status }
}

/// A status word with the given phase tag and nothing else set: success.
pub(crate) fn ok_status(phase: bool) -> u16 {
    phase as u16
}

pub(crate) struct Waited {
    pub result: NvmeResult<()>,
    pub reads: u64,
    pub rung: Vec<u16>,
    pub checks: u64,
}

/// Run the driver's completion wait for command `cid` on submission queue
/// `sq_id` over `ring` (what the controller has written), with the deadline
/// passing after `checks_allowed` checks. Every slot read and doorbell value
/// is checked against the ring as it happens.
pub(crate) fn wait_on(
    cursor: &mut CqCursor,
    sq_id: u16,
    cid: u16,
    ring: &[Completion],
    checks_allowed: u64,
) -> Waited {
    let entries = cursor.entries;
    assert_eq!(ring.len(), entries as usize);
    let reads = Cell::new(0u64);
    let checks = Cell::new(0u64);
    let rung = RefCell::new(Vec::new());
    let result = wait_for_completion(
        cursor,
        sq_id,
        cid,
        |head| {
            assert!(head < entries, "read slot {head} of a {entries}-entry ring");
            reads.set(reads.get() + 1);
            assert!(reads.get() < 50_000_000, "the completion wait did not end");
            ring[head as usize]
        },
        |head| {
            assert!(head < entries, "rang head {head} of a {entries}-entry ring");
            rung.borrow_mut().push(head);
        },
        || {
            checks.set(checks.get() + 1);
            assert!(checks.get() < 1_000_000, "the completion wait did not end");
            checks.get() > checks_allowed
        },
    );
    Waited { result, reads: reads.get(), rung: rung.into_inner(), checks: checks.get() }
}

fn cursor_at(entries: u16, head: u16, phase: bool) -> CqCursor {
    let mut c = CqCursor::new(entries);
    c.head = head;
    c.phase = phase;
    c
}

#[test]
fn an_entry_with_last_passes_phase_is_never_taken() {
    for entries in RINGS {
        for head in 0..entries {
            for phase in [true, false] {
                // Every slot holds a perfect completion for this command, but
                // tagged with the other phase: written on the previous pass.
                let ring = vec![entry(9, 0, head, ok_status(!phase)); entries as usize];
                let mut c = cursor_at(entries, head, phase);
                let w = wait_on(&mut c, 0, 9, &ring, 2);
                assert!(matches!(w.result, Err(NvmeError::ControllerTimeout)));
                assert!(w.rung.is_empty(), "a stale entry was consumed");
                assert_eq!((c.head, c.phase), (head, phase));
            }
        }
    }
}

#[test]
fn an_entry_for_another_command_does_not_complete_this_one() {
    for entries in RINGS {
        for other in [0u16, 1, 6, 8, 0x8007, 0xffff] {
            let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
            ring[0] = entry(other, 0, 0, ok_status(true));
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, 0, 7, &ring, 2);
            assert!(
                matches!(w.result, Err(NvmeError::ControllerTimeout)),
                "cid {other} completed command 7"
            );
            // The stray entry is consumed, and only it.
            assert_eq!(w.rung, vec![1 % entries]);
        }
    }
}

#[test]
fn an_entry_from_another_submission_queue_does_not_complete_this_one() {
    // The admin queue serves SQ 0 and the I/O queue SQ 1. A phase-correct,
    // successful entry with the right command id but another SQ id is not
    // this command's completion.
    for entries in RINGS {
        for (expect, other) in [(0u16, 1u16), (0, 0xffff), (1, 0), (1, 2), (1, 0x8001)] {
            let ring = vec![entry(4, other, 0, ok_status(true)); entries as usize];
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, expect, 4, &ring, 2);
            assert!(
                matches!(w.result, Err(NvmeError::ControllerTimeout)),
                "SQ {other} completed a command issued on SQ {expect}"
            );
            // Every stray entry of this pass is consumed, then the ring is
            // empty until the next pass.
            assert_eq!(w.rung.len(), entries as usize);
            let ring = vec![entry(4, expect, 0, ok_status(true)); entries as usize];
            let mut c = CqCursor::new(entries);
            assert!(wait_on(&mut c, expect, 4, &ring, 2).result.is_ok());
        }
    }
}

#[test]
fn a_late_completion_does_not_wedge_the_queue() {
    // Command 7's wait timed out; its completion lands at the head while the
    // driver waits on command 8, whose own completion follows it.
    for entries in RINGS.into_iter().filter(|&n| n >= 2) {
        for head in 0..entries {
            let mut ring = vec![entry(0, 1, 0, ok_status(false)); entries as usize];
            let next = (head + 1) % entries;
            let next_phase = next != 0;
            ring[head as usize] = entry(7, 1, 0, ok_status(true));
            ring[next as usize] = entry(8, 1, 0, ok_status(next_phase));
            let mut c = cursor_at(entries, head, true);
            let w = wait_on(&mut c, 1, 8, &ring, 2);
            assert!(w.result.is_ok(), "command 8 behind a late 7, head {head} of {entries}");
            let after = (next + 1) % entries;
            assert_eq!(w.rung, vec![next, after]);
            assert_eq!(c.head, after);
        }
    }
}

#[test]
fn a_controller_flooding_stray_entries_still_ends_the_wait() {
    // A hostile controller writes a fresh stray entry, with the phase the
    // driver expects, into every slot the driver reaches. The driver
    // consumes each and still gives up on time.
    for entries in RINGS {
        for allowed in [0u64, 3] {
            let shadow = Cell::new((0u16, true));
            let (reads, checks, rung) = (Cell::new(0u64), Cell::new(0u64), Cell::new(0u64));
            let mut c = CqCursor::new(entries);
            let result = wait_for_completion(
                &mut c,
                0,
                5,
                |head| {
                    reads.set(reads.get() + 1);
                    assert!(reads.get() < 50_000_000, "the completion wait did not end");
                    assert_eq!(head, shadow.get().0);
                    entry(6, 0, 0, ok_status(shadow.get().1))
                },
                |head| {
                    assert!(head < entries);
                    rung.set(rung.get() + 1);
                    let (_, phase) = shadow.get();
                    shadow.set((head, if head == 0 { !phase } else { phase }));
                },
                || {
                    checks.set(checks.get() + 1);
                    checks.get() > allowed
                },
            );
            assert!(matches!(result, Err(NvmeError::ControllerTimeout)));
            assert_eq!(checks.get(), allowed + 1);
            assert_eq!(reads.get(), (allowed + 1) * DEADLINE_CHECK_SPINS as u64);
            assert_eq!(rung.get(), reads.get(), "every stray entry is consumed");
        }
    }
}

#[test]
fn any_status_bit_fails_the_command() {
    for entries in RINGS {
        for phase in [true, false] {
            let head = entries - 1;
            for bit in 1..16 {
                let ring =
                    vec![entry(3, 0, 0, ok_status(phase) | (1 << bit)); entries as usize];
                let mut c = cursor_at(entries, head, phase);
                let w = wait_on(&mut c, 0, 3, &ring, 2);
                assert!(
                    matches!(w.result, Err(NvmeError::AdminCommandFailed)),
                    "status bit {bit} passed"
                );
                // A failed command is still consumed, so the queue moves on.
                assert_eq!(w.rung, vec![0]);
            }
            let ring = vec![entry(3, 0, 0, ok_status(phase)); entries as usize];
            let mut c = cursor_at(entries, head, phase);
            assert!(wait_on(&mut c, 0, 3, &ring, 2).result.is_ok());
        }
    }
}

#[test]
fn the_reported_sq_head_never_moves_the_cursor() {
    for entries in RINGS {
        for head in 0..entries {
            for sq_head in [0, 1, entries - 1, entries, 0x7fff, 0xfffe, 0xffff] {
                let ring = vec![entry(5, 0, sq_head, ok_status(true)); entries as usize];
                let mut c = cursor_at(entries, head, true);
                let w = wait_on(&mut c, 0, 5, &ring, 2);
                assert!(w.result.is_ok());
                let wrapped = head + 1 == entries;
                assert_eq!(c.head, if wrapped { 0 } else { head + 1 }, "sq_head {sq_head}");
                assert_eq!(c.phase, !wrapped, "the phase flips on the wrap only");
                assert_eq!(w.rung, vec![c.head]);
            }
        }
    }
}

#[test]
fn every_wait_ends_once_the_deadline_passes() {
    for entries in RINGS {
        for allowed in [0u64, 1, 5] {
            let ring = vec![entry(1, 0, 0, ok_status(false)); entries as usize];
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, 0, 1, &ring, allowed);
            assert!(matches!(w.result, Err(NvmeError::ControllerTimeout)));
            assert_eq!(w.checks, allowed + 1);
            assert_eq!(w.reads, (allowed + 1) * DEADLINE_CHECK_SPINS as u64);
        }
    }
}

#[test]
fn a_well_behaved_controller_completes_around_the_ring() {
    for entries in RINGS {
        let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
        let (mut tail, mut phase) = (0u16, true);
        let mut c = CqCursor::new(entries);
        for n in 0..(3 * entries as u32 + 1) {
            let cid = (n % 0xfffe + 1) as u16;
            ring[tail as usize] = entry(cid, 0, tail, ok_status(phase));
            tail += 1;
            if tail == entries {
                tail = 0;
                phase = !phase;
            }
            let w = wait_on(&mut c, 0, cid, &ring, 2);
            assert!(w.result.is_ok(), "command {n} of a {entries}-entry ring");
            assert_eq!((c.head, c.phase), (tail, phase));
            assert_eq!(w.rung, vec![tail]);
        }
    }
}
