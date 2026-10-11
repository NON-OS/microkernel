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

//! A map whose entries were written but whose flush failed returned an error,
//! and the broker freed frames those entries still named: the device kept a
//! read and write path into the frames' next owner.

use core::cell::Cell;

use super::map_fail::{after_refusal, MapFail};

#[test]
fn a_map_refused_before_writing_frees_without_asking_for_an_unmap() {
    let asked = Cell::new(false);
    assert_eq!(
        after_refusal(false, || {
            asked.set(true);
            false
        }),
        MapFail::Freeable
    );
    assert!(!asked.get(), "an unmap of a range never mapped would only fail");
}

#[test]
fn an_uncommitted_map_frees_once_an_unmap_takes_the_range_away() {
    assert_eq!(after_refusal(true, || true), MapFail::Freeable);
}

#[test]
fn an_uncommitted_map_the_unmap_cannot_take_back_pins_its_frames() {
    assert_eq!(after_refusal(true, || false), MapFail::Pinned);
}

#[test]
fn the_unmap_is_asked_exactly_once_for_an_uncommitted_map() {
    let asked = Cell::new(0);
    after_refusal(true, || {
        asked.set(asked.get() + 1);
        true
    });
    assert_eq!(asked.get(), 1);
}
