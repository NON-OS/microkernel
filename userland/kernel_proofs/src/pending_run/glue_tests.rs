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

//! Part B's kernel text pinned whole (P1 to P3): the hold that moves a full
//! run out in a block closed before it sends, the drain that sends what it
//! took, and the send that takes its run by value.

use crate::proofs::proofs_8::transport_takeover::text::{between, code, tail};

const PENDING: &str = include_str!("../../../../src/fs/cryptoblock/pending.rs");
const SEND: &str = include_str!("../../../../src/fs/cryptoblock/pending_send.rs");

#[test]
fn p2_hold_moves_a_full_run_out_before_it_sends() {
    let hold = concat!(
        "pub(super) fn hold(at: u64, sector: &[u8; SECTOR_BYTES])",
        " -> Result<(), CryptoBlockError> { loop { let full = { let _quiet = quiet();",
        " let mut run = lock_responsive(&PENDING);",
        " let held = (run.bytes.len() / SECTOR_BYTES) as u64;",
        " if !must_send(run.start, held, at, RUN_SECTORS as u64) { if run.bytes.is_empty() {",
        " run.start = at; run.bytes.reserve_exact(RUN_SECTORS * SECTOR_BYTES); }",
        " run.bytes.extend_from_slice(sector); return Ok(()); } take(&mut run) };",
        " send(full)?; } }",
    );
    assert_eq!(between(PENDING, "pub(super) fn hold(", "pub(super) fn drain("), code(hold));
}

#[test]
fn p3_drain_takes_under_a_guard_that_ends_before_the_send() {
    let drain = concat!(
        "pub(super) fn drain() -> Result<(), CryptoBlockError> {",
        " let full = { let _quiet = quiet(); take(&mut lock_responsive(&PENDING)) }; send(full) }",
    );
    assert_eq!(tail(PENDING, "pub(super) fn drain("), code(drain));
}

#[test]
fn p1_send_takes_its_run_by_value_and_take_empties_the_one_held() {
    let take = concat!(
        "pub(super) fn take(run: &mut Run) -> Run {",
        " Run { start: run.start, bytes: core::mem::take(&mut run.bytes) } }",
    );
    let send = concat!(
        "pub(super) fn send(run: Run) -> Result<(), CryptoBlockError> {",
        " if run.bytes.is_empty() { return Ok(()); }",
        " super::device::write(run.start, &run.bytes).map_err(map_block_error) }",
    );
    assert_eq!(between(SEND, "pub(super) fn take(", "pub(super) fn send("), code(take));
    assert_eq!(between(SEND, "pub(super) fn send(", "pub(super) fn drop_held("), code(send));
}
