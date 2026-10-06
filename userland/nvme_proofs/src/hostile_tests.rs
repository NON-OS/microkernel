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

//! A hostile controller, all at once. One xorshift loop draws register
//! values (CAP, CSTS, the mapped window), completion rings and identify pages
//! and runs the driver's real decisions and loops over them, checking each
//! against an oracle written from the spec apart from the driver: nothing
//! panics, nothing succeeds that should not, and no index leaves its range.
//! A named boundary set then pins the edges one by one.

use core::cell::Cell;

use crate::admin::{
    ready_step, ready_timeout_ms, wait_ready, Completion, ControllerIdentity, CqCursor,
    NamespaceIdentity, ReadyStep, DEADLINE_CHECK_SPINS,
};
use crate::completion_tests::{entry, ok_status, wait_on, RINGS};
use crate::constants::{CSTS_CFS, CSTS_RDY};
use crate::doorbell_tests::info;
use crate::error::NvmeError;
use crate::geometry_tests::{controller_with_mdts, ns_page, DATA_BYTES};
use crate::nvm::NamespaceGeometry;
use crate::ready_tests::{cap_with_to, run, SPEC_CEILING_MS};

const ROUNDS: u64 = 200_000;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn chance(&mut self, one_in: u64) -> bool {
        self.below(one_in) == 0
    }
}

// The spec, written out apart from the driver.

fn spec_ready_ms(cap: u64) -> u64 {
    (((cap >> 24) & 0xff) * 500).max(5_000)
}

fn spec_doorbells_fit(cap: u64, mapped: u64) -> bool {
    let stride = (cap >> 32) & 0xf;
    0x1000 + 3 * (4u64 << stride) + 4 <= mapped
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Ok,
    Failed,
    TimedOut,
    /// CSTS.CFS outlived the reset (the disable wait only).
    Fatal,
}

/// Walk the ring as the spec says a host consumes it: stop at the first entry
/// with last pass's phase; consume every entry this pass wrote; the first one
/// naming this SQ and command id completes it. Returns the outcome, how many
/// entries were consumed, and where the cursor ends.
fn spec_walk(
    ring: &[Completion],
    start: CqCursor,
    sq: u16,
    cid: u16,
) -> (Outcome, usize, u16, bool) {
    let (mut head, mut phase, mut consumed) = (start.head, start.phase, 0);
    loop {
        let e = ring[head as usize];
        if (e.status & 1 != 0) != phase {
            return (Outcome::TimedOut, consumed, head, phase);
        }
        consumed += 1;
        head += 1;
        if head == start.entries {
            head = 0;
            phase = !phase;
        }
        if e.sq_id == sq && e.cid == cid {
            let outcome = if e.status >> 1 == 0 { Outcome::Ok } else { Outcome::Failed };
            return (outcome, consumed, head, phase);
        }
    }
}

fn spec_geometry(mdts: u8, nn: u32, page: &[u8; 4096]) -> Option<(u64, u32, u32)> {
    let nsze = u64::from_le_bytes(page[0..8].try_into().ok()?);
    let nlbaf = page[0x19];
    let flbas = page[0x1a];
    let index = (flbas & 0x0f) as usize;
    let ms = u16::from_le_bytes([page[0x80 + index * 4], page[0x81 + index * 4]]);
    let lbads = page[0x82 + index * 4];
    let usable = nn != 0
        && nsze != 0
        && matches!(lbads, 9 | 12)
        && ms == 0
        && flbas & 0x60 == 0
        && index as u16 <= nlbaf as u16;
    if !usable {
        return None;
    }
    let lba = 1u64 << lbads;
    let limit =
        if mdts == 0 || mdts >= 52 { DATA_BYTES } else { (4096u64 << mdts).min(DATA_BYTES) };
    Some((nsze, lba as u32, (limit / lba) as u32))
}

/// What bring-up records for NSID 1: absent when NN is 0, the page otherwise.
fn identify(nn: u32, page: &[u8; 4096]) -> NamespaceIdentity {
    if nn == 0 {
        NamespaceIdentity::absent()
    } else {
        NamespaceIdentity::parse(1, page)
    }
}

#[test]
fn a_hostile_controller_never_panics_never_passes_and_never_indexes_out_of_range() {
    let mut r = Rng(0x0bad_c0de_dead_beef);
    // Reached, waiting, fatal, timed out; completed, failed, timed out;
    // namespaces taken.
    let mut seen = [0u64; 8];
    for round in 0..ROUNDS {
        // Registers. Half the CSTS values are all ones or carry CFS.
        let cap = r.next();
        let csts = match r.below(4) {
            0 => u32::MAX,
            1 => r.next() as u32 | CSTS_CFS,
            _ => r.next() as u32 & !CSTS_CFS,
        };
        let want = r.chance(2);
        let elapsed = if r.chance(2) { r.below(140_000) } else { r.next() };
        let bound = ready_timeout_ms(cap);
        assert!(bound <= SPEC_CEILING_MS && bound == spec_ready_ms(cap));
        let fatal = csts & CSTS_CFS != 0;
        let gone = csts == u32::MAX;
        let short = fatal || (csts & CSTS_RDY != 0) != want;
        match ready_step(cap, csts, want, elapsed) {
            ReadyStep::Reached => {
                assert!(!fatal && (csts & CSTS_RDY != 0) == want);
                seen[0] += 1;
            }
            ReadyStep::Wait => {
                assert!(short && !(gone || (want && fatal)));
                assert!(elapsed < bound);
                seen[1] += 1;
            }
            ReadyStep::Failed(NvmeError::UnsupportedController) => {
                assert!(gone || (want && fatal));
                seen[2] += 1;
            }
            ReadyStep::Failed(NvmeError::ControllerTimeout) => {
                assert!(short && !fatal && !gone);
                assert!(elapsed >= bound);
                seen[3] += 1;
            }
            // Only the disable wait outlives a CFS, and only to its bound.
            ReadyStep::Failed(NvmeError::ControllerFatal) => {
                assert!(!want && fatal && !gone);
                assert!(elapsed >= bound);
                seen[3] += 1;
            }
            ReadyStep::Failed(e) => panic!("the ready wait failed with {e:?}"),
        }
        let mapped = if r.chance(2) { r.below(0x80) << 12 } else { r.next() };
        assert_eq!(info(cap).doorbells_fit(mapped), spec_doorbells_fit(cap, mapped));

        // The real ready loop, every 64th round, against a controller that
        // reports a fresh random CSTS on every read.
        if round % 64 == 0 {
            let tick = 1 + r.below(997);
            let seed = Cell::new(r.next() | 1);
            let last = Cell::new(0u32);
            let now = Cell::new(0u64);
            let reads = Cell::new(0u64);
            let result = wait_ready(
                cap,
                want,
                || {
                    let mut s = Rng(seed.get());
                    let v = s.next();
                    seed.set(s.0);
                    // Mostly plain "not there yet", now and then anything.
                    let c = if v.is_multiple_of(50) {
                        (v >> 8) as u32
                    } else {
                        ((v >> 8) as u32 & !3) | (!want as u32)
                    };
                    last.set(c);
                    reads.set(reads.get() + 1);
                    assert!(reads.get() < 10_000_000, "the ready wait did not end");
                    c
                },
                || {
                    let t = now.get();
                    now.set(t + tick);
                    Some(t)
                },
            );
            let c = last.get();
            match result {
                Ok(()) => assert!(c & CSTS_CFS == 0 && (c & CSTS_RDY != 0) == want),
                Err(NvmeError::UnsupportedController) => {
                    assert!(c == u32::MAX || (want && c & CSTS_CFS != 0))
                }
                Err(NvmeError::ControllerTimeout) => assert!(now.get() >= bound),
                Err(NvmeError::ControllerFatal) => {
                    assert!(!want && c & CSTS_CFS != 0 && now.get() >= bound)
                }
                Err(e) => panic!("the ready loop failed with {e:?}"),
            }
            assert!(now.get() <= bound + 2 * tick, "the ready loop ran past its bound");
        }

        // A completion ring the controller filled at random, entries leaning
        // toward this SQ and command id and toward a clean status. Half the
        // rounds plant the awaited command at the head with this pass's
        // phase.
        let entries = RINGS[r.below(4) as usize];
        let sq = r.below(2) as u16;
        let cid = r.next() as u16;
        let mut ring: Vec<Completion> = (0..entries)
            .map(|_| {
                let v = r.next();
                let mut e = Completion {
                    dw0: v as u32,
                    dw1: (v >> 32) as u32,
                    sq_head: (v >> 7) as u16,
                    sq_id: if v & 0x300 == 0 { sq } else { (v >> 23) as u16 & 3 },
                    cid: if v & 0xc00 == 0 { cid } else { (v >> 40) as u16 },
                    status: (v >> 48) as u16,
                };
                if v & 0x3000 == 0 {
                    e.status &= 1;
                }
                e
            })
            .collect();
        let mut cursor = CqCursor::new(entries);
        cursor.head = r.below(entries as u64) as u16;
        cursor.phase = r.chance(2);
        if r.chance(2) {
            let at = cursor.head as usize;
            ring[at].sq_id = sq;
            ring[at].cid = cid;
            ring[at].status = (ring[at].status & !1) | ok_status(cursor.phase);
        }
        let start = cursor;
        let (outcome, consumed, head, phase) = spec_walk(&ring, start, sq, cid);
        let w = wait_on(&mut cursor, sq, cid, &ring, 0);
        let got = match w.result {
            Ok(()) => Outcome::Ok,
            Err(NvmeError::AdminCommandFailed) => Outcome::Failed,
            Err(NvmeError::ControllerTimeout) => Outcome::TimedOut,
            Err(e) => panic!("the completion wait failed with {e:?}"),
        };
        assert_eq!(got, outcome, "round {round}");
        assert_eq!(w.rung.len(), consumed, "round {round}");
        assert_eq!((cursor.head, cursor.phase), (head, phase), "round {round}");
        assert!(cursor.head < entries);
        if outcome == Outcome::TimedOut {
            assert_eq!(w.reads, DEADLINE_CHECK_SPINS as u64);
        }
        seen[4 + got as usize] += 1;

        // Identify pages: the bytes the decision reads, drawn at random and
        // pulled toward the edges; every 64th page is random throughout.
        let mdts = if r.chance(4) { r.below(4) as u8 } else { r.next() as u8 };
        let nn = if r.chance(8) { 0 } else { r.next() as u32 | 1 };
        let nsze = if r.chance(8) { 0 } else { r.next() };
        let formats: Vec<(u16, u8)> = (0..16)
            .map(|_| {
                let ms = if r.chance(4) { r.next() as u16 } else { 0 };
                let lbads = if r.chance(4) { r.next() as u8 } else { [9, 12][r.below(2) as usize] };
                (ms, lbads)
            })
            .collect();
        let nlbaf = if r.chance(2) { r.below(16) as u8 } else { r.next() as u8 };
        let flbas = if r.chance(4) { r.next() as u8 } else { r.below(16) as u8 };
        let mut page = ns_page(nsze, nlbaf, flbas, &formats);
        if round % 64 == 1 {
            for b in page.iter_mut() {
                *b = r.next() as u8;
            }
        }
        let ns = identify(nn, &page);
        assert!(ns.format_index < 16, "a format slot past the sixteen parsed");
        let mut id_page = [0u8; 4096];
        id_page[0x4d] = mdts;
        id_page[0x204..0x208].copy_from_slice(&nn.to_le_bytes());
        let id = ControllerIdentity::parse(&id_page);
        let got = NamespaceGeometry::check(&id, &ns).ok().map(|g| {
            assert!(g.max_sectors >= 1);
            assert!(g.max_sectors as u64 * g.lba_size as u64 <= DATA_BYTES);
            (g.capacity_sectors, g.lba_size, g.max_sectors)
        });
        assert_eq!(got, spec_geometry(mdts, nn, &page), "round {round}");
        seen[7] += got.is_some() as u64;
    }
    // Every branch was reached often enough to mean something.
    for (i, n) in seen.iter().enumerate() {
        assert!(*n > ROUNDS / 100, "branch {i} reached only {n} times");
    }
}

// The named boundary set.

#[test]
fn named_ready_wait_boundaries() {
    // (what, CAP.TO, waiting for RDY =, CSTS at ms, outcome)
    type Csts = fn(u64) -> u32;
    let cases: [(&str, u8, bool, Csts, Outcome); 12] = [
        ("CAP.TO 0, enable, CSTS never flips", 0, true, |_| 0, Outcome::TimedOut),
        ("CAP.TO 0, disable, CSTS never flips", 0, false, |_| CSTS_RDY, Outcome::TimedOut),
        ("CAP.TO 255, enable, CSTS never flips", 255, true, |_| 0, Outcome::TimedOut),
        ("CAP.TO 255, disable, CSTS never flips", 255, false, |_| CSTS_RDY, Outcome::TimedOut),
        ("CAP.TO 0, ready at once", 0, true, |_| CSTS_RDY, Outcome::Ok),
        ("CFS with RDY set while enabling", 20, true, |_| CSTS_CFS | CSTS_RDY, Outcome::Failed),
        ("CFS that never clears while disabling", 20, false, |_| CSTS_CFS, Outcome::Fatal),
        ("all ones while enabling", 20, true, |_| u32::MAX, Outcome::Failed),
        ("all ones while disabling", 20, false, |_| u32::MAX, Outcome::Failed),
        (
            "CFS raised after a second of waiting",
            20,
            true,
            |ms| if ms < 1_000 { 0 } else { CSTS_CFS },
            Outcome::Failed,
        ),
        (
            "ready 1 ms before CAP.TO 20 runs out",
            20,
            true,
            |ms| if ms < 9_999 { 0 } else { CSTS_RDY },
            Outcome::Ok,
        ),
        (
            "ready 1 ms after CAP.TO 20 runs out",
            20,
            true,
            |ms| if ms < 10_001 { 0 } else { CSTS_RDY },
            Outcome::TimedOut,
        ),
    ];
    for (what, to, want, csts, expect) in cases {
        let cap = cap_with_to(u64::MAX, to);
        let r = run(cap, want, 1, csts);
        let got = match r.result {
            Ok(()) => Outcome::Ok,
            Err(NvmeError::UnsupportedController) => Outcome::Failed,
            Err(NvmeError::ControllerTimeout) => Outcome::TimedOut,
            Err(NvmeError::ControllerFatal) => Outcome::Fatal,
            Err(e) => panic!("{what}: {e:?}"),
        };
        assert_eq!(got, expect, "{what}");
        assert!(r.elapsed_ms <= ready_timeout_ms(cap) + 2, "{what}: ran to {} ms", r.elapsed_ms);
    }
}

#[test]
fn named_completion_boundaries() {
    // (what, SQ the command was issued on, the entry at the head, outcome)
    let ok = ok_status(true);
    let cases: [(&str, u16, Completion, Outcome); 12] = [
        ("phase tag from the last pass", 0, entry(5, 0, 1, ok_status(false)), Outcome::TimedOut),
        ("command id one past the issued one", 0, entry(6, 0, 1, ok), Outcome::TimedOut),
        ("command id zero", 1, entry(0, 1, 1, ok), Outcome::TimedOut),
        ("SQ id 1 on the admin queue", 0, entry(5, 1, 1, ok), Outcome::TimedOut),
        ("SQ id 0 on the I/O queue", 1, entry(5, 0, 1, ok), Outcome::TimedOut),
        ("success with SQ head 0xffff", 1, entry(5, 1, 0xffff, ok), Outcome::Ok),
        (
            "success with garbage in DW0 and DW1",
            0,
            Completion { dw0: !0, dw1: !0, ..entry(5, 0, 0, ok) },
            Outcome::Ok,
        ),
        ("SC 1, invalid opcode", 0, entry(5, 0, 1, ok | (1 << 1)), Outcome::Failed),
        ("SCT 7, vendor specific", 1, entry(5, 1, 1, ok | (7 << 9)), Outcome::Failed),
        ("CRD set alone", 0, entry(5, 0, 1, ok | (3 << 12)), Outcome::Failed),
        ("More set alone", 1, entry(5, 1, 1, ok | (1 << 14)), Outcome::Failed),
        ("DNR set alone", 0, entry(5, 0, 1, ok | (1 << 15)), Outcome::Failed),
    ];
    for (what, sq, e, expect) in cases {
        for entries in RINGS {
            let mut ring = vec![entry(0, 0, 0, ok_status(false)); entries as usize];
            ring[0] = e;
            let mut c = CqCursor::new(entries);
            let w = wait_on(&mut c, sq, 5, &ring, 0);
            let got = match w.result {
                Ok(()) => Outcome::Ok,
                Err(NvmeError::AdminCommandFailed) => Outcome::Failed,
                Err(NvmeError::ControllerTimeout) => Outcome::TimedOut,
                Err(err) => panic!("{what}: {err:?}"),
            };
            assert_eq!(got, expect, "{what}, {entries}-entry ring");
            assert!(c.head < entries);
        }
    }
}

#[test]
fn named_identify_boundaries() {
    // (what, MDTS, NN, NSZE, NLBAF, FLBAS, (metadata, LBADS) at the
    //  selected slot, sectors per command or None for no I/O queue)
    type Case = (&'static str, u8, u32, u64, u8, u8, (u16, u8), Option<u32>);
    let cases: [Case; 16] = [
        ("MDTS 0 (no limit), 512-byte LBAs", 0, 1, 1 << 20, 0, 0, (0, 9), Some(64)),
        ("MDTS 0 (no limit), 4096-byte LBAs", 0, 1, 1 << 20, 0, 0, (0, 12), Some(8)),
        ("MDTS 1 (8 KiB)", 1, 1, 1 << 20, 0, 0, (0, 9), Some(16)),
        ("MDTS 255", 255, 1, 1 << 20, 0, 0, (0, 12), Some(8)),
        ("NN 0", 0, 0, 1 << 20, 0, 0, (0, 9), None),
        ("NSZE 0", 0, 1, 0, 0, 0, (0, 9), None),
        ("NSZE all ones", 0, 1, u64::MAX, 0, 0, (0, 9), Some(64)),
        ("LBADS 8", 0, 1, 1 << 20, 0, 0, (0, 8), None),
        ("LBADS 13", 0, 1, 1 << 20, 0, 0, (0, 13), None),
        ("LBADS 32", 0, 1, 1 << 20, 0, 0, (0, 32), None),
        ("LBADS 255", 0, 1, 1 << 20, 0, 0, (0, 255), None),
        ("8 bytes of metadata inside each block", 0, 1, 1 << 20, 0, 0x10, (8, 9), None),
        ("8 bytes of metadata in a separate buffer", 0, 1, 1 << 20, 0, 0, (8, 9), None),
        ("FLBAS 1 with NLBAF 0", 0, 1, 1 << 20, 0, 1, (0, 9), None),
        ("FLBAS 15 with NLBAF 15", 0, 1, 1 << 20, 15, 15, (0, 9), Some(64)),
        ("FLBAS index bits 6:5 set", 0, 1, 1 << 20, 63, 0x20, (0, 9), None),
    ];
    for (what, mdts, nn, nsze, nlbaf, flbas, format, expect) in cases {
        let mut formats = [(0xffffu16, 255u8); 16];
        formats[(flbas & 0x0f) as usize] = format;
        let page = ns_page(nsze, nlbaf, flbas, &formats);
        let g = NamespaceGeometry::check(&controller_with_mdts(mdts), &identify(nn, &page)).ok();
        assert_eq!(g.as_ref().map(|g| g.max_sectors), expect, "{what}");
        if let Some(g) = g {
            assert_eq!(g.capacity_sectors, nsze, "{what}");
        }
    }
}
