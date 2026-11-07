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

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState};
use super::rig::{press, H, W};
use crate::input::InputKind;

const SCREEN: (u32, u32) = (1920, 1080);

fn step(state: &mut DragState, reads: &mut u32, answer: Option<(u32, u32)>) -> Option<(u32, u32)> {
    state.display_or(|| {
        *reads += 1;
        answer
    })
}

fn release(state: &mut DragState, x: i32, y: i32) {
    let _ = drag::handle(state, (W, H), (100, 100), false, &event(InputKind::ButtonUp, x, y));
}

#[test]
fn a_drag_asks_the_display_size_once_for_all_its_steps() {
    let mut state = DragState::new();
    let _ = press(&mut state, 200, 10);
    let mut reads = 0;
    for _ in 0..64 {
        assert_eq!(step(&mut state, &mut reads, Some(SCREEN)), Some(SCREEN));
    }
    assert_eq!(reads, 1, "every step of one drag asked the compositor again");
}

#[test]
fn a_failed_read_is_asked_again_on_the_next_step() {
    let mut state = DragState::new();
    let _ = press(&mut state, 200, 10);
    let mut reads = 0;
    assert_eq!(step(&mut state, &mut reads, None), None);
    assert_eq!(step(&mut state, &mut reads, Some(SCREEN)), Some(SCREEN));
    assert_eq!(step(&mut state, &mut reads, Some((1, 1))), Some(SCREEN));
    assert_eq!(reads, 2);
}

#[test]
fn each_press_and_release_reads_the_display_afresh() {
    let mut state = DragState::new();
    let mut reads = 0;
    let _ = press(&mut state, 200, 10);
    assert_eq!(step(&mut state, &mut reads, Some(SCREEN)), Some(SCREEN));
    release(&mut state, 260, 40);
    // A display resized between two drags: the next drag clamps to the new size.
    assert_eq!(step(&mut state, &mut reads, Some((1280, 800))), Some((1280, 800)));
    let _ = press(&mut state, 200, 10);
    assert_eq!(step(&mut state, &mut reads, Some((2560, 1440))), Some((2560, 1440)));
    assert_eq!(reads, 3);
}
