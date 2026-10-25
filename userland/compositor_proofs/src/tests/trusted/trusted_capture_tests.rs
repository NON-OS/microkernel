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

//! What a screenshot holds, composed by the compositor's own `capture` from
//! surfaces in memory: every client's pixels as the screen shows them, but
//! never a pixel of the trusted band, even where a prompt covers a window;
//! never the cursor; and no more than both the canvas and the shot hold.

use crate::tests::trusted::trusted_desk::{shoot, APP, H, PROMPT, W};
use crate::trusted_rule::TRUSTED_Z;

#[test]
fn a_prompt_on_screen_is_never_in_the_capture() {
    for ceiling in [u32::MAX, TRUSTED_Z, TRUSTED_Z + 5] {
        let (screen, shot, got) = shoot(W, H, ceiling);
        assert_eq!(got, (W, H));
        assert_eq!(screen[(5 * W + 8) as usize], PROMPT, "the screen shows the prompt");
        assert!(
            shot.iter().all(|&p| p == APP),
            "the shot shows the window under it, and no cursor"
        );
    }
}

#[test]
fn the_capture_goes_as_far_as_both_hold_and_no_further() {
    let (_, shot, got) = shoot(10, 20, u32::MAX);
    assert_eq!(got, (10, H));
    assert!(shot[..(10 * H) as usize].iter().all(|&p| p == APP));
    assert!(shot[(10 * H) as usize..].iter().all(|&p| p == 0xEE), "nothing past the canvas");
}
