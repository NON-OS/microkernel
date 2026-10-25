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


//! The kept list's own code, pinned whole (whole), that kept_world and
//! kept_count_tests drive through the kept rule (L78, L80): a run kept
//! waiting, not for good; the let-go's word on it; the bound's count of a
//! function's frames kept for good alone; and the settle keyed by the wipe's
//! own claim, freeing and keeping just as the rule says, each free noted.

use super::code::flat;
use super::whole::whole;

const KEPT: &str = include_str!("../../../../../src/hardware/broker/dma/kept.rs");
const SETTLE: &str = include_str!("../../../../../src/hardware/broker/dma/kept_settle.rs");

const KEEP: [&str; 3] = [
    "pub(super) fn keep(device_id: u64, epoch: u64, start: u64, len: u64) {",
    "let (run, for_good) = (Run { let_go: false, settled: false }, false);",
    "KEPT.lock().push(Kept { device_id, epoch, start, len, run, for_good }); }",
];

const DEFER: [&str; 5] = [
    "pub(super) fn defer(start: u64, len: u64) -> Option<bool> { let mut all = KEPT.lock();",
    "let at = all.iter().position(|k| (k.start, k.len) == (start, len))?;",
    "let (run, act) = on_let_go(all[at].run);",
    "if act == Act::Free { all.remove(at); return Some(false); }",
    "all[at].run = run; Some(true) }",
];

const BYTES: [&str; 2] = [
    "pub(in crate::hardware::broker) fn kept_bytes(device_id: u64) -> u64 { let all = KEPT.lock();",
    "all.iter().filter(|k| k.device_id == device_id && k.for_good).map(|k| k.len).sum() }",
];

const SETTLES: [&str; 10] = [
    "pub(in crate::hardware::broker) fn settle_kept( device_id: u64, epoch: u64, frees: bool, )",
    "-> (usize, usize) { let (mut go, mut kept) = (Vec::new(), 0usize);",
    "KEPT.lock().retain_mut(|k| {",
    "if (k.device_id, k.epoch) != (device_id, epoch) { return true; }",
    "let (run, act) = on_settle(k.run, frees); k.run = run;",
    "match act { Act::Free => go.push((k.start, k.len)),",
    "Act::Keep => { k.for_good = true; kept += 1; } Act::Wait => {} } act != Act::Free });",
    "for &(start, len) in &go { super::held_run::free_now(start, len);",
    "crate::memory::iommu::note_unconfined_released(1); }",
    "(go.len(), kept) }",
];

#[test]
fn the_kept_list_keeps_lets_go_counts_and_settles_as_the_rule_says() {
    let k = flat(KEPT);
    whole(&k, "keep", &KEEP);
    whole(&k, "defer", &DEFER);
    whole(&k, "kept_bytes", &BYTES);
    whole(&flat(SETTLE), "settle_kept", &SETTLES);
}
