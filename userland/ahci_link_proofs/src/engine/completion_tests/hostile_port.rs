// NONOS Operating System (AGPL-3.0-or-later)
//! A port whose every register read is drawn afresh: what the wait on a
//! command may have read last when it ends the way it does.

use super::hostile::{draw, expect, rounds, seeded, FATAL_IS, LINK, OFS};
use super::model_port::{rounds_of, Snap, RUNNING_ENGINE};
use crate::constants::regs::{PORT_CI, PORT_CMD, PORT_IS, PORT_SACT, PORT_TFD};
use crate::engine::completion::wait_done;
use crate::error::AhciError;

/// The device changes its mind between any two reads.
pub(super) struct HostilePort {
    pub s: u64,
    pub log: Vec<(u32, u32)>,
}

impl HostilePort {
    pub fn read(&mut self, off: u32) -> u32 {
        let clean = match off {
            PORT_TFD => 0x50,
            PORT_CMD => RUNNING_ENGINE,
            _ => 0,
        };
        let v = draw(&mut self.s, clean);
        self.log.push((off, v));
        v
    }

    /// The last `n` reads, oldest first.
    pub fn tail(&self, n: usize) -> Vec<(u32, u32)> {
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
        assert_eq!(
            regs,
            [PORT_CI, PORT_SACT, PORT_IS, PORT_TFD],
            "seed {seed}: one look per round"
        );
        let snap = Snap { ci: last[0].1, sact: last[1].1, is: last[2].1, tfd: last[3].1 };
        match got {
            Ok(()) => {
                assert_eq!(snap.ci, 0, "seed {seed}: done with a slot still set");
                assert_eq!(snap.sact, 0, "seed {seed}: done with a queued slot");
                assert_eq!(
                    snap.is & (FATAL_IS | OFS | LINK),
                    0,
                    "seed {seed}: done on a fatal PxIS"
                );
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
