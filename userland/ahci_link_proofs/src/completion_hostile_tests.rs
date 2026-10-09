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

//! A hostile port: whatever PxCI, PxSACT, PxIS and PxTFD read, and whenever
//! they change, the wait ends in Done only on a clean status read after a
//! clear slot bit. Anything else is an error, never a served sector.

use crate::completion_tests::{
    rounds_of, ModelPort, Snap, CLEAN_DONE, LIMIT, RUNNING, RUNNING_STALE,
};
use crate::constants::regs::{PORT_CI, PORT_IS, PORT_SACT, PORT_TFD};
use crate::engine::completion::{verdict, wait_done, wait_ready, Verdict};
use crate::error::AhciError;

const FATAL_IS: u32 = 0x7800_0000;
const OFS: u32 = 1 << 24;

fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        300
    } else {
        full
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn seeded(seed: u64) -> u64 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1
}

fn done(before: Snap, after: Snap, done_after: usize) -> Result<(), AhciError> {
    let mut port = ModelPort::new(before, after, done_after);
    wait_done(|off| port.read(off), rounds_of(LIMIT))
}

/// How the wait must end on a port that shows `s` from some read onward,
/// restated from the AHCI bits apart from the driver's source.
fn expect(s: Snap) -> Result<(), AhciError> {
    let failed = s.is & (FATAL_IS | OFS) != 0 || s.tfd & 0x01 != 0 || s.ci & !1 != 0 || s.sact != 0;
    if failed {
        return Err(AhciError::CommandFailed);
    }
    if s.ci & 1 != 0 || s.tfd & 0x88 != 0 {
        return Err(AhciError::Timeout);
    }
    Ok(())
}

/// One register value, clean three times in four so every outcome is
/// reached, anything at all otherwise.
fn draw(s: &mut u64, clean: u32) -> u32 {
    let r = xorshift(s);
    match r & 7 {
        0 | 1 => xorshift(s) as u32,
        2 => clean | (1 << ((r >> 8) % 32)),
        _ => clean,
    }
}

fn hostile_snap(s: &mut u64) -> Snap {
    Snap { ci: draw(s, 0), sact: draw(s, 0), is: draw(s, 0x1), tfd: draw(s, 0x50) }
}

#[test]
fn an_error_posted_with_the_completion_is_never_done() {
    /*
     * The device fails the command and the HBA clears the slot bit with
     * it, as older QEMU releases do. Wherever the completion lands between
     * two reads, ERR is seen.
     */
    let failed = [
        Snap { ci: 0, sact: 0, is: 1 << 30, tfd: 0x0451 },
        Snap { ci: 0, sact: 0, is: 0, tfd: 0x51 },
    ];
    for before in [RUNNING, RUNNING_STALE] {
        for after in failed {
            for done_after in 0..24 {
                assert_eq!(
                    done(before, after, done_after),
                    Err(AhciError::CommandFailed),
                    "{before:?} then {after:?} after {done_after} reads"
                );
            }
        }
    }
}

#[test]
fn a_slot_cleared_while_the_device_shows_bsy_or_drq_is_not_done() {
    for tfd in [0x80u32, 0xd0, 0x08, 0x58, 0x88] {
        let unsettled = Snap { ci: 0, sact: 0, is: 1, tfd };
        for done_after in 0..12 {
            assert_eq!(done(RUNNING, unsettled, done_after), Err(AhciError::Timeout), "{tfd:#x}");
        }
        assert_eq!(done(unsettled, CLEAN_DONE, 9), Ok(()), "{tfd:#x} settling to idle is done");
    }
}

#[test]
fn a_slot_the_driver_did_not_issue_fails_the_command() {
    for bit in 1..32 {
        let stray = Snap { ci: 1 << bit, sact: 0, is: 1, tfd: 0x50 };
        assert_eq!(done(RUNNING, stray, 3), Err(AhciError::CommandFailed), "PxCI bit {bit}");
        let both = Snap { ci: 1 | 1 << bit, sact: 0, is: 0, tfd: 0x80 };
        assert_eq!(done(RUNNING, both, 3), Err(AhciError::CommandFailed), "slot 0 and {bit}");
    }
}

#[test]
fn a_queued_command_the_driver_never_issued_fails() {
    for bit in 0..32 {
        let queued = Snap { ci: 0, sact: 1 << bit, is: 1, tfd: 0x50 };
        assert_eq!(done(RUNNING, queued, 3), Err(AhciError::CommandFailed), "PxSACT bit {bit}");
    }
}

#[test]
fn an_overflow_fails_the_command() {
    let overflow = Snap { ci: 0, sact: 0, is: OFS | 1, tfd: 0x50 };
    for done_after in 0..12 {
        assert_eq!(done(RUNNING, overflow, done_after), Err(AhciError::CommandFailed));
    }
}

#[test]
fn a_port_with_work_outstanding_is_not_issued_to() {
    for (ci, sact) in [(1u32, 0u32), (1 << 7, 0), (u32::MAX, 0), (0, 1), (0, 1 << 31)] {
        let busy = Snap { ci, sact, is: 0, tfd: 0x50 };
        let mut port = ModelPort::new(busy, busy, usize::MAX);
        let got = wait_ready(|off| port.read(off), rounds_of(LIMIT));
        assert_eq!(got, Err(AhciError::CommandFailed), "PxCI {ci:#x} PxSACT {sact:#x}");
    }
}

#[test]
fn the_verdict_matches_the_bits_on_every_look() {
    for seed in 1..rounds(200_000) {
        let mut s = seeded(seed);
        let snap = hostile_snap(&mut s);
        let want = match expect(snap) {
            Ok(()) => Verdict::Done,
            Err(AhciError::CommandFailed) => Verdict::Failed,
            Err(_) => Verdict::Pending,
        };
        assert_eq!(verdict(snap.ci, snap.sact, snap.is, snap.tfd), want, "seed {seed}: {snap:?}");
    }
}

#[test]
fn the_wait_ends_as_the_final_status_says_wherever_it_lands() {
    let mut reached = [0u64; 3];
    for seed in 1..rounds(100_000) {
        let mut s = seeded(seed);
        let after = hostile_snap(&mut s);
        let running_tfd = if xorshift(&mut s) & 1 == 0 { 0x80 } else { 0x50 };
        let before = Snap { ci: 1, sact: 0, is: xorshift(&mut s) as u32 & 0x7, tfd: running_tfd };
        let done_after = (xorshift(&mut s) % 40) as usize;
        let mut port = ModelPort::new(before, after, done_after);
        let got = wait_done(|off| port.read(off), rounds_of(64));
        assert_eq!(got, expect(after), "seed {seed}: {after:?} after {done_after} reads");
        reached[match got {
            Ok(()) => 0,
            Err(AhciError::CommandFailed) => 1,
            Err(_) => 2,
        }] += 1;
    }
    if !cfg!(miri) {
        assert!(reached.iter().all(|&n| n > 1_000), "outcomes reached: {reached:?}");
    }
}

/// A port whose every register read is drawn afresh: the device changes
/// its mind between any two reads.
struct HostilePort {
    s: u64,
    log: Vec<(u32, u32)>,
}

impl HostilePort {
    fn read(&mut self, off: u32) -> u32 {
        let clean = if off == PORT_TFD { 0x50 } else { 0 };
        let v = draw(&mut self.s, clean);
        self.log.push((off, v));
        v
    }

    /// The last `n` reads, oldest first.
    fn tail(&self, n: usize) -> Vec<(u32, u32)> {
        self.log[self.log.len().saturating_sub(n)..].to_vec()
    }
}

#[test]
fn done_only_on_a_clean_status_read_after_a_clear_slot() {
    let (mut ok, mut failed) = (0u64, 0u64);
    for seed in 1..rounds(100_000) {
        let mut port = HostilePort { s: seeded(seed), log: Vec::new() };
        let got = wait_done(|off| port.read(off), rounds_of(16));
        let last = port.tail(4);
        let regs: Vec<u32> = last.iter().map(|&(off, _)| off).collect();
        assert_eq!(regs, [PORT_CI, PORT_SACT, PORT_IS, PORT_TFD], "seed {seed}: one look per round");
        let snap = Snap { ci: last[0].1, sact: last[1].1, is: last[2].1, tfd: last[3].1 };
        match got {
            Ok(()) => {
                assert_eq!(snap.ci, 0, "seed {seed}: done with a slot still set");
                assert_eq!(snap.sact, 0, "seed {seed}: done with a queued slot");
                assert_eq!(snap.is & (FATAL_IS | OFS), 0, "seed {seed}: done on a fatal PxIS");
                assert_eq!(snap.tfd & 0x89, 0, "seed {seed}: done on ERR, BSY or DRQ");
                ok += 1;
            }
            Err(AhciError::CommandFailed) => {
                assert_eq!(expect(snap), Err(AhciError::CommandFailed), "seed {seed}: {snap:?}");
                failed += 1;
            }
            Err(e) => {
                assert_eq!(e, AhciError::Timeout, "seed {seed}");
                assert_eq!(port.log.len(), 4 * 16, "seed {seed}: gave up before the limit");
            }
        }
    }
    if !cfg!(miri) {
        assert!(ok > 1_000 && failed > 1_000, "done {ok}, failed {failed}");
    }
}

#[test]
fn issued_to_only_when_idle_with_nothing_outstanding() {
    let mut ok = 0u64;
    for seed in 1..rounds(100_000) {
        let mut port = HostilePort { s: seeded(seed), log: Vec::new() };
        if wait_ready(|off| port.read(off), rounds_of(16)) == Ok(()) {
            let last = port.tail(3);
            assert_eq!(last[0].0, PORT_TFD, "seed {seed}");
            assert_eq!(last[0].1 & 0x88, 0, "seed {seed}: issued into BSY or DRQ");
            assert_eq!(last[1], (PORT_CI, 0), "seed {seed}: issued over a set slot");
            assert_eq!(last[2], (PORT_SACT, 0), "seed {seed}: issued over a queued slot");
            ok += 1;
        }
    }
    if !cfg!(miri) {
        assert!(ok > 1_000, "issued {ok} times");
    }
}

#[test]
fn every_register_read_is_one_the_wait_names() {
    let mut port = HostilePort { s: seeded(3), log: Vec::new() };
    let _ = wait_done(|off| port.read(off), rounds_of(64));
    let _ = wait_ready(|off| port.read(off), rounds_of(64));
    assert!(port.log.iter().all(|&(off, _)| [PORT_CI, PORT_SACT, PORT_IS, PORT_TFD].contains(&off)));
}
