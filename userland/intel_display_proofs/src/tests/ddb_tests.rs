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

//! The display buffer split between pipes, planes and cursors.

use crate::engine::ddb::{allocate, Run, BLOCKS};

#[test]
fn one_pipe_takes_the_whole_buffer_and_a_larger_cursor_share() {
    let a = allocate([true, false, false]);
    assert_eq!(a[0], (Run { start: 0, end: 864 }, Run { start: 864, end: BLOCKS }));
    assert!(a[1].0.is_empty() && a[2].1.is_empty());
}

#[test]
fn three_pipes_share_evenly_without_overlap() {
    let a = allocate([true, true, true]);
    assert_eq!(a[0], (Run { start: 0, end: 290 }, Run { start: 290, end: 298 }));
    assert_eq!(a[2], (Run { start: 596, end: 886 }, Run { start: 886, end: 894 }));
    let b = allocate([false, true, true]);
    assert_eq!(b[1].0.start, 0);
    assert_eq!(b[2].1.end, BLOCKS);
}

#[test]
fn buffer_config_words_hold_the_inclusive_end() {
    assert_eq!(Run { start: 290, end: 298 }.cfg(), (297 << 16) | 290);
    assert_eq!(Run::default().cfg(), 0);
    assert!(allocate([false; 3]).iter().all(|r| r.0.is_empty() && r.1.is_empty()));
}

#[test]
fn a_run_reads_back_from_its_register_value() {
    for run in [Run { start: 0, end: 448 }, Run { start: 448, end: 888 }, Run::default()] {
        assert_eq!(Run::of(run.cfg()), run);
    }
    assert_eq!(Run::of((863 << 16) | 400), Run { start: 400, end: 864 });
}
