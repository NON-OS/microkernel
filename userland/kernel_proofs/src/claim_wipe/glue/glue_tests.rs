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

//! The kernel glue the models stand for, read where they cannot run it
//! (L78). A kept run's entry is made before the surface registry is asked,
//! so a let-go on another CPU finds it and waits; a run the registry does
//! not hold is let go through the same entry. Each job a step takes reads
//! the clock afresh, and each reset write is stamped once it is made, so no
//! quiet window opens before its write. The kept frames' bound counts only
//! runs kept for good.

use super::code::code;

const TEARDOWN: &str = include_str!("../../../../../src/hardware/broker/dma/teardown.rs");
const KEPT: &str = include_str!("../../../../../src/hardware/broker/dma/kept.rs");
const SETTLE: &str = include_str!("../../../../../src/hardware/broker/dma/kept_settle.rs");
const STEP: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/step.rs");
const RESET: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/reset.rs");
const ACT: &str = include_str!("../../../../../src/hardware/broker/claim/wipe/act.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn a_kept_run_is_entered_before_the_registry_is_asked() {
    let t = code(TEARDOWN);
    let kept = &t[at(&t, "if super::kept_rule::keeps(")..];
    let branch = &kept[..at(kept, "        return;")];
    let (entry, ask) = (at(branch, "super::kept::keep("), at(branch, "grant_run::hold("));
    assert!(entry < ask, "the entry made after the registry was asked");
    assert!(branch[ask..].contains("super::kept::defer("), "a run nobody holds never let go");
}

#[test]
fn the_bound_counts_only_runs_kept_for_good() {
    let (k, s) = (code(KEPT), code(SETTLE));
    let bytes = &k[at(&k, "fn kept_bytes(")..];
    assert!(bytes[..at(bytes, "\n}")].contains("k.for_good"), "waiting runs counted");
    let keep = &s[at(&s, "Act::Keep =>")..];
    assert!(keep[..at(keep, "}")].contains("k.for_good = true"), "kept for good never marked");
}

#[test]
fn each_job_reads_the_clock_and_each_write_is_stamped_once_made() {
    let (s, r, a) = (code(STEP), code(RESET), code(ACT));
    let jobs = &s[at(&s, "for job in jobs")..];
    let job = &jobs[..at(jobs, "\n    }")];
    assert!(job.find("now_ns()").unwrap_or(usize::MAX) < at(job, "act::act("), "a stale clock");
    let write = &r[at(&r, "pub(super) fn write(")..];
    let (made, stamp) = (at(write, "let _ = match m {"), at(write, "w.phase = written(m, "));
    assert!(
        made < stamp && write[stamp..].starts_with("w.phase = written(m, crate::time::now_ns()")
    );
    let d0 = &a[at(&a, "fn to_d0(")..];
    assert!(at(d0, "cfg.write16(") < at(d0, "Phase::D0 { at: crate::time::now_ns()"));
}
