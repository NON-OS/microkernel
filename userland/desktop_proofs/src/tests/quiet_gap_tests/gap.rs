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

//! How long a quiet service is let be.

use crate::quiet_gap::QuietGap;

#[test]
fn a_service_that_answers_is_asked_on_every_tick() {
    let mut gap = QuietGap::new();
    for now in [0u64, 1_000, 2_000, 3_000] {
        assert!(gap.due(now));
        gap.answered();
    }
}

/* One unanswered read stops the next ones until the gap has passed, so a
 * quiet policy store costs one timeout, not one every second. */
#[test]
fn a_quiet_service_is_let_be_for_a_doubling_gap() {
    let mut gap = QuietGap::new();
    gap.missed(10_000);
    assert!(!gap.due(10_500));
    assert!(gap.due(10_000 + QuietGap::FIRST_MS));
    gap.missed(11_000);
    assert!(!gap.due(12_999));
    assert!(gap.due(13_000), "the second gap is twice the first");
    gap.missed(13_000);
    assert!(gap.due(17_000) && !gap.due(16_999));
}

#[test]
fn the_gap_stops_growing_at_its_ceiling() {
    let mut gap = QuietGap::new();
    for now in 0..20u64 {
        gap.missed(now);
    }
    assert!(!gap.due(19 + QuietGap::MAX_MS - 1));
    assert!(gap.due(19 + QuietGap::MAX_MS));
}
