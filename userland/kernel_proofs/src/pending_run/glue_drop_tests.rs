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

//! drop_held pinned whole (P4): it clears the run under PENDING, and the
//! count moves when anything was dropped or the writer was swept.

use crate::proofs::proofs_8::transport_takeover::text::{code, tail};

const SEND: &str = include_str!("../../../../src/fs/cryptoblock/pending_send.rs");
const DEFERRED: &str = include_str!("../../../../src/fs/cryptoblock/write_deferred.rs");

#[test]
fn p4_drop_held_clears_and_moves_the_count_when_it_dropped_or_swept() {
    let clear = concat!(
        "pub(super) fn drop_held() -> bool { let _quiet = crate::services::lifecycle::quiet();",
        " let mut run = crate::smp::lock_responsive(&PENDING); let had = !run.bytes.is_empty();",
        " run.bytes = Vec::new(); had }",
    );
    let leave = concat!(
        "pub fn drop_held(swept: bool) { let dropped = super::pending_send::drop_held();",
        " if dropped && !SAID.swap(true, Ordering::Relaxed) {",
        " crate::log::error!(\"[CRYPTOBLOCK] a writer left sectors held; dropped unsent\"); }",
        " if dropped || swept { super::epoch::advance(); } }",
    );
    assert_eq!(tail(SEND, "pub(super) fn drop_held("), code(clear));
    assert_eq!(tail(DEFERRED, "pub fn drop_held("), code(leave));
}
