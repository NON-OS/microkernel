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
//! Subscribers hear that focus moved whenever it did, by whatever path:
//! A window's owner, and only its owner, hears that its window took or lost
//! the keyboard (focus/notice.rs keys_notices, server/keys_notify.rs): one
//! notice for the window that lost it and one for the one that took it,
//! loser first, none when focus stays, and never one naming another pid's
//! window to a pid.

use crate::focus::model::FocusedRef;
use crate::focus::notice::keys_notices;

const SEND: &str = include_str!("../../capsule_wm/src/server/keys_notify.rs");
const TELL: &str = include_str!("../../capsule_wm/src/server/focus_notify.rs");

fn at(owner_pid: u32, window_id: u32) -> Option<FocusedRef> {
    Some(FocusedRef { owner_pid, window_id })
}

#[test]
fn the_loser_then_the_taker_and_nothing_when_focus_stays() {
    assert_eq!(keys_notices(at(40, 7), at(41, 9)), [Some((40, 7, false)), Some((41, 9, true))]);
    assert_eq!(keys_notices(at(40, 7), at(40, 8)), [Some((40, 7, false)), Some((40, 8, true))]);
    assert_eq!(keys_notices(None, at(41, 9)), [None, Some((41, 9, true))]);
    assert_eq!(keys_notices(at(40, 7), None), [Some((40, 7, false)), None]);
    assert_eq!(keys_notices(at(40, 7), at(40, 7)), [None, None]);
    assert_eq!(keys_notices(None, None), [None, None]);
}

#[test]
fn each_notice_names_its_owners_window_and_no_other() {
    let ws = [None, at(40, 7), at(40, 8), at(41, 9), at(42, 7)];
    for was in ws {
        for now in ws {
            let pair = |f: Option<FocusedRef>| f.map(|f| (f.owner_pid, f.window_id));
            for (pid, window, has) in keys_notices(was, now).into_iter().flatten() {
                let whose = pair(if has { now } else { was });
                assert_eq!(whose, Some((pid, window)), "{:?} to {:?}", pair(was), pair(now));
            }
        }
    }
}

#[test]
fn the_notice_goes_to_the_owner_alone_on_the_focus_edge() {
    assert!(SEND.contains("mk_ipc_send_to_pid(pid, frame.as_ptr()"), "to the named owner");
    assert!(SEND.contains("NOTIFY_KIND_KEYS, pid, window_id, has as u32"), "the owner's kind");
    assert!(!SEND.contains("subscriptions"), "not the subscribers");
    let call = TELL.find("keys_notify::tell_owners(was, ctx.focus.current());").expect("told");
    assert!(call < TELL.find("broadcast(").expect("the broadcast"), "on the same edge");
}
