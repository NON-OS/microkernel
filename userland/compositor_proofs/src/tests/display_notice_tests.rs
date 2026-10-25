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

//! H-09: the input router hears the display size whenever it changes, at
//! once, retried until a notice goes, and again now and then.

use crate::state::display_rule::{due, REFRESH_MS, RETRY_MS};

const HD: (u32, u32) = (1920, 1080);
const GOP: (u32, u32) = (1024, 768);

#[test]
fn a_new_mode_is_told_at_once() {
    assert!(due(HD, Some(GOP), 0, RETRY_MS), "the mode change waits for nothing but a retry gap");
    assert!(due(HD, None, 0, RETRY_MS), "the first size");
}

#[test]
fn a_size_already_told_is_not_told_again_until_the_refresh() {
    assert!(!due(HD, Some(HD), 0, RETRY_MS));
    assert!(!due(HD, Some(HD), 0, REFRESH_MS - 1));
    assert!(due(HD, Some(HD), 0, REFRESH_MS), "a router started since hears it");
}

#[test]
fn a_notice_that_did_not_go_is_tried_again_after_the_retry_gap_only() {
    // The last attempt failed (told stayed GOP): not again within RETRY_MS.
    assert!(!due(HD, Some(GOP), 1_000, 1_000 + RETRY_MS - 1));
    assert!(due(HD, Some(GOP), 1_000, 1_000 + RETRY_MS));
}

#[test]
fn no_display_is_never_told() {
    for size in [(0, 0), (0, 1080), (1920, 0)] {
        assert!(!due(size, Some(GOP), 0, REFRESH_MS * 2), "{size:?}");
        assert!(!due(size, None, 0, REFRESH_MS * 2), "{size:?}");
    }
}
