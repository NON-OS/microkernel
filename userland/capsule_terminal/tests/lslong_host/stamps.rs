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

//! The mtime stamp: epoch, padding, known dates and width.

use crate::ls_date::stamp;
use crate::support::at;

#[test]
fn the_epoch_itself_renders() {
    assert_eq!(at(1), "Jan  1 00:00");
}

/// The reason the day is space-padded rather than zero-padded: single digit
/// days must line up under the two digit ones without reading as octal.
#[test]
fn single_digit_days_are_space_padded() {
    assert_eq!(at(1_755_000_000_000), "Aug 12 12:00");
    assert_eq!(at(1_754_006_400_000), "Aug  1 00:00");
}

#[test]
fn known_epochs_render_their_civil_date() {
    for (ms, want) in [
        (1_755_594_600_000u64, "Aug 19 09:10"),
        (946_684_800_000, "Jan  1 00:00"),
        (1_000_000_000_000, "Sep  9 01:46"),
        (1_234_567_890_000, "Feb 13 23:31"),
        (2_147_483_647_000, "Jan 19 03:14"),
    ] {
        assert_eq!(at(ms), want, "{ms}");
    }
}

/// Every stamp is the same width or the name column walks.
#[test]
fn every_stamp_is_twelve_bytes_including_the_unknown_one() {
    assert_eq!(stamp(0).len(), 12);
    assert_eq!(at(0), "           -");
    for ms in [1u64, 1_755_594_600_000, 4_107_542_400_000, 253_370_764_800_000] {
        assert_eq!(stamp(ms).len(), 12, "{ms}");
    }
}
