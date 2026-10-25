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
//! The splash following the compositor's canvas (follow.rs): when it asks,
//! which answers count as a move, and how many redraws it allows.

use crate::follow::{Follow, CHECK_MS};

const FW: (u32, u32, u32) = (1280, 800, 5120);
const GPU: (u32, u32, u32) = (1920, 1080, 7680);

#[test]
fn asks_every_quarter_second() {
    assert_eq!(CHECK_MS, 250);
    let f = Follow::new(FW, 1000);
    assert!(!f.due(1000) && !f.due(1249));
    assert!(f.due(1250) && f.due(9000));
}

#[test]
fn a_clock_gone_back_asks_at_once() {
    let f = Follow::new(FW, 1000);
    assert!(f.due(999));
}

#[test]
fn a_new_size_is_a_move_once() {
    let mut f = Follow::new(FW, 0);
    assert_eq!(f.seen(250, Ok(GPU)), Some(GPU));
    assert_eq!(f.size, GPU);
    // Asked again at the same size: no move, and the wait starts again.
    assert_eq!(f.seen(500, Ok(GPU)), None);
    assert!(!f.due(600) && f.due(750));
    assert_eq!(f.redraws, 1);
}

#[test]
fn a_stride_change_alone_is_a_move() {
    let mut f = Follow::new(FW, 0);
    assert_eq!(f.seen(250, Ok((1280, 800, 5632))), Some((1280, 800, 5632)));
}
