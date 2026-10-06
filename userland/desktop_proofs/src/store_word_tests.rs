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

use crate::store_state::store_word::store_word;
use crate::store_state::NotifyLevel;
use crate::toast_state::toast::TOAST_TEXT_MAX;
use nonos_app_skeleton::clients::vfs::{store_was_read, NO_DISK};

/* A store that loaded with one entry refused (9) is a disk that works: a
 * warning, never "corrupted". */
#[test]
fn one_refused_entry_is_a_warning_not_corruption() {
    let (text, level) = store_word(9).expect("said");
    assert!(level == NotifyLevel::Warn);
    assert!(!text.windows(9).any(|w| w == b"corrupted"));
    assert!(store_was_read(9), "and a disk was read");
}

#[test]
fn a_store_read_and_not_decoded_is_an_error() {
    for code in [3, 6] {
        let (_, level) = store_word(code).expect("said");
        assert!(level == NotifyLevel::Error, "code {code}");
    }
}

/* A healthy store, and codes vfs never latches at boot, say nothing. */
#[test]
fn a_healthy_store_says_nothing() {
    for code in [0, 8, 10, 11, 12] {
        assert!(store_word(code).is_none(), "code {code}");
    }
}

/* A stick carries its store: one that did not answer, or refused the read,
 * is a warning, and no disk at all says what the boot lacks. */
#[test]
fn a_store_that_did_not_load_is_said() {
    for code in [2, 7] {
        let (_, level) = store_word(code).expect("said");
        assert!(level == NotifyLevel::Warn, "code {code}");
    }
    let (text, level) = store_word(1).expect("said");
    assert!(level == NotifyLevel::Info);
    assert!(text.windows(5).any(|w| w == b"Linux"), "it says what is missing");
}

/* A toast holds TOAST_TEXT_MAX bytes and cuts the rest: every line is whole. */
#[test]
fn every_line_fits_a_toast_whole() {
    for code in 0..=12 {
        if let Some((text, _)) = store_word(code) {
            assert!(text.len() <= TOAST_TEXT_MAX, "code {code}: {} bytes", text.len());
        }
    }
}

#[test]
fn only_the_four_no_disk_codes_read_as_no_disk() {
    for code in 0..=12 {
        assert_eq!(store_was_read(code), !NO_DISK.contains(&code), "code {code}");
    }
    assert!(!store_was_read(1) && !store_was_read(2) && !store_was_read(3) && !store_was_read(7));
    assert!(store_was_read(0) && store_was_read(9));
}
