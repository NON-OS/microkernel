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

//! How long an answer is kept.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::upstream::{answer, fresh, resolve_a, script, Datagram};

const DAY_MS: i64 = 86_400_000;

/// The upstream answers with `first` until told otherwise, every time with
/// `ttl`.
fn serve(ttl: u32) -> Arc<AtomicU32> {
    let ip = Arc::new(AtomicU32::new(u32::from_be_bytes([192, 0, 2, 1])));
    let shared = ip.clone();
    script(move |q| {
        let now = shared.load(Ordering::SeqCst).to_be_bytes();
        vec![Datagram::from_server(answer(q, now, ttl))]
    });
    ip
}

/*
 * A TTL of 2^31 - 1 seconds kept one answer for 68 years: one forged reply
 * that won a race, or one mistaken record, was never asked for again. A day
 * is the most any answer is kept.
 */
#[test]
fn no_answer_is_kept_longer_than_a_day() {
    let _g = fresh();
    let ip = serve(0x7FFF_FFFF);
    assert_eq!(resolve_a("example.com"), (0, Some([192, 0, 2, 1])));
    ip.store(u32::from_be_bytes([192, 0, 2, 2]), Ordering::SeqCst);
    nonos_libc::advance(DAY_MS + 1_000);
    assert_eq!(resolve_a("example.com"), (0, Some([192, 0, 2, 2])), "asked again after a day");
}

#[test]
fn an_answer_is_kept_for_its_ttl() {
    let _g = fresh();
    let ip = serve(300);
    assert_eq!(resolve_a("example.com"), (0, Some([192, 0, 2, 1])));
    ip.store(u32::from_be_bytes([192, 0, 2, 2]), Ordering::SeqCst);
    nonos_libc::advance(200_000);
    assert_eq!(resolve_a("example.com"), (0, Some([192, 0, 2, 1])), "still fresh");
    nonos_libc::advance(200_000);
    assert_eq!(resolve_a("example.com"), (0, Some([192, 0, 2, 2])), "expired");
}
