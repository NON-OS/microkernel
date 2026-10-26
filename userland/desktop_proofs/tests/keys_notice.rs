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
//! An app believes the WM's word that its window took or lost the keyboard
//! (app_skeleton runner/inbox/keys.rs): the WM's notice of kind 4 naming
//! this window, from the WM's pid alone, and nothing else.

use nonos_app_skeleton::runner::inbox::keys::keys_for;

const DRAIN: &str = include_str!("../../app_skeleton/src/runner/inbox/drain.rs");
const APPLY: &str = include_str!("../../app_skeleton/src/runner/apply_drain.rs");

fn notice(kind: u32, window: u32, x: u32) -> Vec<u8> {
    let mut m = b"VMWN".to_vec();
    m.extend_from_slice(&1u16.to_le_bytes());
    m.extend_from_slice(&0u16.to_le_bytes());
    for w in [kind, 41, window, x, 0] {
        m.extend_from_slice(&w.to_le_bytes());
    }
    m
}

#[test]
fn keys_notice_for_this_window_says_taken_or_lost() {
    assert_eq!(keys_for(&notice(4, 9, 1), 9), Some(true));
    assert_eq!(keys_for(&notice(4, 9, 0), 9), Some(false));
}

#[test]
fn keys_notice_is_no_other_message() {
    assert_eq!(keys_for(&notice(4, 8, 1), 9), None, "another window");
    assert_eq!(keys_for(&notice(3, 9, 1), 9), None, "the broadcast kind");
    assert_eq!(keys_for(&notice(4, 9, 2), 9), None, "neither taken nor lost");
    let mut v2 = notice(4, 9, 1);
    v2[4] = 2;
    assert_eq!(keys_for(&v2, 9), None, "another version");
    let mut magic = notice(4, 9, 1);
    magic[0] = b'X';
    assert_eq!(keys_for(&magic, 9), None, "another magic");
    assert_eq!(keys_for(&notice(4, 9, 1)[..27], 9), None, "short");
    assert_eq!(keys_for(&[], 9), None, "empty");
}

#[test]
fn keys_notice_believed_from_the_wm_alone_and_kept() {
    assert!(DRAIN.contains("keys_for(msg, window_id).filter(|_| from_wm(sender))"));
    let keys = DRAIN.find("keys_for(").unwrap();
    assert!(keys < DRAIN.find("handle_control(").unwrap(), "before anything else reads it");
    assert!(APPLY.contains("booted.focused = has;"), "the window keeps it");
}
