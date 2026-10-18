// NONOS Operating System (AGPL-3.0-or-later)
//! The wait before issue: on a port whose every read is drawn afresh, what
//! it read last when it lets a command through; and on a port that holds
//! work the driver never gave it, no command at all.

use super::hostile::{rounds, seeded};
use super::hostile_port::HostilePort;
use super::model_port::{rounds_of, ModelPort, Snap, LIMIT, RUNNING_ENGINE};
use crate::constants::regs::{PORT_CI, PORT_CMD, PORT_IS, PORT_SACT, PORT_TFD};
use crate::engine::completion::wait_done;
use crate::engine::ready::wait_ready;
use crate::error::AhciError;

#[test]
fn issued_to_only_when_idle_with_nothing_outstanding() {
    let mut ok = 0u64;
    for seed in 1..rounds(100_000) {
        let mut port = HostilePort { s: seeded(seed), log: Vec::new() };
        if wait_ready(|off| port.read(off), rounds_of(16)) == Ok(()) {
            let last = port.tail(4);
            assert_eq!((last[0].0, last[1].0), (PORT_CMD, PORT_TFD), "seed {seed}");
            let engine = last[0].1 & RUNNING_ENGINE;
            assert_eq!(engine, RUNNING_ENGINE, "seed {seed}: issued with the engine off");
            assert_eq!(last[1].1 & 0x88, 0, "seed {seed}: issued into BSY or DRQ");
            assert_eq!(last[2], (PORT_CI, 0), "seed {seed}: issued over a set slot");
            assert_eq!(last[3], (PORT_SACT, 0), "seed {seed}: issued over a queued slot");
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
    let named = [PORT_CMD, PORT_CI, PORT_SACT, PORT_IS, PORT_TFD];
    assert!(port.log.iter().all(|&(off, _)| named.contains(&off)));
}

#[test]
fn a_port_with_work_outstanding_is_not_issued_to() {
    for (ci, sact) in [(1u32, 0u32), (1 << 7, 0), (u32::MAX, 0), (0, 1), (0, 1 << 31)] {
        let busy = Snap { ci, sact, is: 0, tfd: 0x50 };
        let mut port = ModelPort::new(busy, busy, usize::MAX);
        let got = wait_ready(|off| port.read(off), rounds_of(LIMIT));
        assert_eq!(got, Err(AhciError::HbaFailed), "PxCI {ci:#x} PxSACT {sact:#x}");
    }
}
