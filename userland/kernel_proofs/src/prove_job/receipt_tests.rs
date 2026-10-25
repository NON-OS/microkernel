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

//! RECEIPT (letter agent-02): asked for only by a launcher, the Terminal or
//! the shell, holding SpawnWindow; one waits at a time, after PROVE and
//! DELIVER; and the job is told it serves a receipt.

use super::ask::{ask_for, Caller, ProveAsk, RECEIPT};
use super::ask_tests::caller;
use super::job_argv::{job_argv, JobKind};
use super::slots::{Next, Slots};

#[test]
fn only_a_launcher_asks_for_a_receipt() {
    let launcher = Caller { launcher: true, ..caller(8, false, false, false) };
    assert_eq!(ask_for(RECEIPT, &launcher), Some(ProveAsk::Receipt(8)));
    for c in [caller(8, true, true, true), caller(8, false, false, false)] {
        assert_eq!(ask_for(RECEIPT, &c), None, "the store, a Vendor or a supervisor is not one");
    }
}

#[test]
fn a_receipt_waits_once_after_prove_and_deliver() {
    let mut s = Slots::new();
    assert!(s.ask(ProveAsk::Receipt(8), 0));
    assert!(!s.ask(ProveAsk::Receipt(9), 0), "one RECEIPT waits already");
    assert!(s.ask(ProveAsk::Prove(3), 0));
    assert!(s.waiting());
    assert_eq!(s.next(), Some(Next::Prove(3)));
    assert_eq!(s.next(), Some(Next::Receipt(8)));
    assert_eq!(s.next(), None);
    assert!(!s.waiting());
}

#[test]
fn the_receipt_job_is_told_its_op_and_its_pid() {
    assert_eq!(job_argv(JobKind::Receipt, 77), ["--nonos-prove-job", "receipt", "77"]);
}

/// Only a RECEIPT job asks for AttestRead, bit 31, on top of the prover's own
/// caps, and Capsule.mk keeps the prover's word and offers bit 31 as optional.
#[test]
fn only_a_receipt_job_holds_attest_read() {
    use super::job_argv::job_caps;
    const PROVE: u64 = 0xC_0000_1879;
    const ATTEST_READ: u64 = 1 << 31;
    assert_eq!(job_caps(JobKind::Receipt, PROVE, ATTEST_READ), PROVE | ATTEST_READ);
    for k in [JobKind::Prove, JobKind::Deliver] {
        assert_eq!(job_caps(k, PROVE, ATTEST_READ), PROVE, "no AttestRead for {k:?}");
    }
    let mk = include_str!("../../../capsule_prove/Capsule.mk");
    let word = "CAPSULE_REQUIRED_CAPS    := 0xC00001879\n";
    assert!(mk.contains(word), "the prover's word, unchanged");
    assert!(mk.contains("CAPSULE_OPTIONAL_CAPS    := 0x80000100\n"), "Debug, and bit 31");
    let defs = include_str!("../../../../src/capabilities/types/defs.rs");
    assert!(defs.contains("AttestRead = 1 << 31,"));
    let job = include_str!("../../../../src/userspace/capsule_prove/job.rs");
    let asks = "requested_caps: job_caps(kind, PROVE_CAPS, ATTEST_READ) | serial_debug_cap(),";
    assert!(job.contains(asks));
}
