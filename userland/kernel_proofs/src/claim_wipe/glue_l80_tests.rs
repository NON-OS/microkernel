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

//! The orders L80 rests on, read where the models cannot run them. An end
//! settles its kept frames before it is written; a claim asks the kept bound
//! only once the record is read, inside the claims and wipes holds; libc
//! asks again for a time, not a count of sleeps, as long as the model's.

use super::model_attempt::RESET_MS;
use super::phase_rule::MS;

const FINISH: &str = include_str!("../../../../src/hardware/broker/claim/wipe/finish.rs");
const ASK: &str = include_str!("../../../../src/hardware/broker/claim/wipe/ask.rs");
const ENTER: &str = include_str!("../../../../src/hardware/broker/claim/enter.rs");
const LIBC: &str = include_str!("../../../libc/src/broker/device.rs");

fn flat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn an_end_settles_its_frames_before_it_is_written() {
    let f = flat(FINISH);
    let put = &f[at(&f, "pub(super) fn put(")..at(&f, "fn say(")];
    let settle = at(put, "Some(e) => crate::hardware::broker::dma::settle_kept(");
    assert!(settle < at(put, "lock_responsive(&WIPES)"), "the end written before the settle");
    assert!(put.contains("None | Some(End::Lost) => (0, 0),"), "a lost end settled its frames");
}

#[test]
fn the_kept_bound_is_asked_after_the_record_inside_both_holds() {
    let a = flat(ASK);
    let admits = &a[at(&a, "fn admits(")..at(&a, "fn rearm(")];
    let (lock, record) = (at(admits, "let all = WIPES.lock();"), at(admits, "let record ="));
    assert!(lock < record && record < at(admits, "over_cap()"), "the bound asked too early");
    let e = flat(ENTER);
    let body = &e[at(&e, "pub(super) fn enter(")..];
    let claims = at(body, "let mut claims = claims();");
    assert!(claims < at(body, "kept_bytes(device_id) >= KEEP_CAP"), "bound read before the hold");
    assert!(body.contains("wipe::admits(device_id, f.by, capped)"), "the bound not asked late");
    assert_eq!(body.matches("kept_bytes(").count(), 1, "the bound read twice");
}

#[test]
fn libc_asks_again_for_as_long_as_the_model_says() {
    let l = flat(LIBC);
    let claim = &l[at(&l, "pub extern \"C\" fn mk_device_claim(")..];
    let deadline = at(claim, "let until = crate::time::Deadline::after_ms(RESET_MS);");
    assert!(deadline < at(claim, "if got != EAGAIN || until.expired() {"), "a count bounds it");
    let ms = &l[at(&l, "const RESET_MS: u64 = ") + 22..];
    let ms: u64 = ms[..at(ms, ";")].parse().expect("RESET_MS a number");
    assert_eq!(ms * MS, RESET_MS, "libc and the model disagree on the bound");
}
