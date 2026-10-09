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

//! More than one finger, a palm, and the click button on the gesture engine.

use super::frames::{button, finger, palm, two, LIFTED};
use crate::input::TouchGesture;

#[test]
fn two_fingers_scroll_and_the_pointer_stays_put_until_both_lift() {
    let mut g = TouchGesture::default();
    assert_eq!(g.on_touch(&two(300, 400)).wheel, 0, "the first frame only anchors the scroll");
    assert_eq!(
        g.on_touch(&two(300, 450)).wheel,
        -2,
        "50 units down an 800-unit pad is two notches, scrolling the view down"
    );
    assert_eq!(
        g.on_touch(&finger(300, 440)).motion,
        None,
        "the remaining finger moved the pointer"
    );
    g.on_touch(&LIFTED);
    g.on_touch(&finger(300, 300));
    assert!(g.on_touch(&finger(340, 300)).motion.is_some(), "the pointer stayed frozen after lift");
}

#[test]
fn a_palm_freezes_the_pointer_until_it_lifts() {
    let mut g = TouchGesture::default();
    g.on_touch(&palm(300, 300));
    assert_eq!(g.on_touch(&palm(340, 300)).motion, None);
    assert_eq!(
        g.on_touch(&finger(380, 300)).motion,
        None,
        "a palm became a finger without lifting"
    );
    g.on_touch(&LIFTED);
    g.on_touch(&finger(300, 300));
    assert!(g.on_touch(&finger(340, 300)).motion.is_some());
}

#[test]
fn the_click_button_is_debounced_over_two_frames() {
    let mut g = TouchGesture::default();
    assert!(!g.on_touch(&button(true)).button_down, "one frame clicked");
    assert!(g.on_touch(&button(true)).button_down);
    g.on_touch(&button(false));
    assert!(g.on_touch(&button(false)).button_up);
}

/// A finger of a hybrid-mode report: one contact, the frame's count only on
/// the first report of each frame.
fn hybrid(id: u32, y: u32, count: u32) -> crate::hid::TouchSample {
    let mut f = two(300 + id * 200, y);
    f.contacts = count;
    f.contact_id = Some(id);
    f
}

#[test]
fn hybrid_reports_scroll_by_the_first_finger_and_keep_the_rest_between_notches() {
    let mut g = TouchGesture::default();
    let mut wheel = 0;
    // Two fingers 200 units apart moving down together, 10 units a frame:
    // finger 0 carries the count, finger 1 says zero.
    for step in 0..10u32 {
        let y = 300 + step * 10;
        wheel += g.on_touch(&hybrid(0, y, 2)).wheel;
        wheel += g.on_touch(&hybrid(1, y + 120, 0)).wheel;
    }
    // 90 units of travel at 25 units a notch: three whole notches, down.
    assert_eq!(wheel, -3, "the second finger's reports reset or moved the scroll");
}

#[test]
fn natural_scrolling_is_off_and_moving_up_scrolls_up() {
    let mut g = TouchGesture::default();
    g.on_touch(&two(300, 500));
    assert_eq!(g.on_touch(&two(300, 450)).wheel, 2, "fingers moving up scroll the view up");
}

#[test]
fn a_torn_jump_while_scrolling_re_anchors_instead_of_flinging() {
    let mut g = TouchGesture::default();
    g.on_touch(&two(300, 100));
    assert_eq!(g.on_touch(&two(300, 700)).wheel, 0, "a jump of most of the pad is not a swipe");
    assert_eq!(g.on_touch(&two(300, 725)).wheel, -1, "the scroll goes on from the new anchor");
}

#[test]
fn a_pad_without_contact_identifiers_still_scrolls_on_its_count() {
    let mut g = TouchGesture::default();
    let mut f = two(300, 300);
    f.contact_id = None;
    g.on_touch(&f);
    f.y = 350;
    assert_eq!(g.on_touch(&f).wheel, -2);
}
