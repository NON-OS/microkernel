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

//! Router input the shell reads after the unlock that was posted while the
//! passphrase was checked (state/lock/released.rs): the lock's, dropped
//! ahead of every other handler, so a click on the lock's field or a chord
//! typed during the check does not act on the desktop once it shows.

use crate::shell_lock::released::{is_locks, posted_ns, released_at};
use crate::shell_lock::Lock;

const MS: u64 = 1_000_000;
const CHECK: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/check.rs");

fn frame(posted: u64) -> [u8; 48] {
    let mut f = [0u8; 48];
    f[0..4].copy_from_slice(&0x4E49_4E50u32.to_le_bytes());
    f[32..40].copy_from_slice(&posted.to_le_bytes());
    f
}

#[test]
fn a_frame_reads_its_post_time_where_the_router_writes_it() {
    let router = include_str!("../../../../capsule_input_router/src/protocol/delivery.rs");
    assert!(router.contains("out[32..40].copy_from_slice(&event.timestamp_ns.to_le_bytes());"));
    assert_eq!(posted_ns(&frame(4_321 * MS + 7)), 4_321 * MS + 7);
    assert_eq!(posted_ns(&[0u8; 39]), 0, "a short frame carries none");
    assert_eq!(posted_ns(&[]), 0);
}

#[test]
fn input_posted_by_the_release_is_the_locks_and_later_input_is_not() {
    let at = Some(released_at(5_000));
    assert_eq!(at, Some(5_000 * MS));
    assert!(is_locks(at, 4_200 * MS), "typed while the keyring checked");
    assert!(is_locks(at, 5_000 * MS + MS - 1), "the release's own millisecond");
    assert!(!is_locks(at, 5_001 * MS), "after the release");
    assert!(!is_locks(at, 0), "a frame with no time is taken as posted now");
}

#[test]
fn an_undated_frame_is_judged_as_the_router_judges_it() {
    let router = include_str!("../../../../capsule_input_router/src/route/session_lock.rs");
    assert!(router.contains("posted_ns != 0 && posted_ns / NS_PER_MS <= ms"));
    let at = Some(released_at(5_000));
    assert!(!is_locks(at, posted_ns(&[0u8; 39])), "a short frame is not dropped forever");
    assert!(is_locks(at, 1), "a frame dated at the boot's start is the lock's");
}

#[test]
fn nothing_is_the_locks_before_the_first_release() {
    assert_eq!(Lock::default().released_ns, None);
    assert!(!is_locks(None, 0));
    assert!(!is_locks(None, u64::MAX));
}

#[test]
fn the_release_time_saturates_and_never_wraps() {
    assert_eq!(released_at(-5), 0);
    assert_eq!(released_at(i64::MAX), u64::MAX);
}

#[test]
fn the_filter_drops_it_ahead_of_the_shell_and_dates_the_release_after_the_router() {
    let filter = CHECK.find("pub fn filter(").expect("filter");
    let dropped =
        CHECK.find("if is_locks(ctx.lock.released_ns, posted_ns(buf)) {").expect("dropped");
    let end = CHECK[filter..].find("\n}\n").map(|e| filter + e).expect("filter ends");
    assert!(filter < dropped && dropped < end, "the drop is in the lock's filter");
    let release = CHECK.find("fn release(").expect("release");
    let told = CHECK[release..].find("super::router::tell(ctx);").expect("told") + release;
    let dated = CHECK[release..]
        .find("ctx.lock.released_ns = Some(released_at(mk_uptime_ms()));")
        .expect("dated")
        + release;
    assert!(told < dated, "the shell dates its release once the router has dated its unlock");
    let frame = include_str!("../../../../capsule_desktop_shell/src/server/input/frame.rs");
    let lock = frame.find("crate::server::lock::filter(ctx, kind, buf)").expect("lock filter");
    let attention = frame.find("handlers::attention::filter(").expect("attention");
    let key = frame.find("super::key::key(").expect("keys");
    let pointer = frame.find("super::pointer::pointer(").expect("pointer");
    assert!(lock < attention && lock < key && lock < pointer, "the lock's filter comes first");
}

#[test]
fn the_prompt_dates_its_close_so_input_queued_while_the_keyring_set_is_dropped() {
    let prompt = include_str!("../../../../capsule_desktop_shell/src/server/lock/prompt.rs");
    let close = prompt.find("fn close(ctx: &mut Context) {").expect("close");
    let end = prompt[close..].find("\n}\n").expect("close ends") + close;
    let body = &prompt[close..end];
    let gone = body.find("ctx.lock.choose = None;").expect("the prompt goes");
    let dated = body
        .find("ctx.lock.released_ns = Some(released_at(mk_uptime_ms()));")
        .expect("its close is dated");
    assert!(gone < dated, "dated as the prompt goes");
    let save = prompt.find("fn save(").expect("save");
    let set = prompt[save..].find("keyring_client::set(").expect("the keyring sets") + save;
    let closed = prompt[set..].find("close(ctx);").expect("closed after the set") + set;
    assert!(set < closed, "the close that drops the queued input follows the keyring's set");
}
