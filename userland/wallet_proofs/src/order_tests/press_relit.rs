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

//! The frame is painted again exactly when a guarded screen's presses arm
//! or disarm (arm/tick.rs), the second passing with nothing else changed
//! among them, so a greyed press never stays greyed once it would act.

use super::press_grey::{GUARDED, LOOKS, T};
use crate::order::terms::arming::ARM_MS;
use crate::wallet::arm::look::{painted, pressed, seen, Arm};
use crate::wallet::arm::moves::Shown;
use crate::wallet::arm::shows::{greys, relit, shows_armed};
use crate::wallet::screen::hits::Press::{Back, Footer};

/// Whether each look of LOOKS paints the frame again.
const DIRTY: [bool; 7] = [false, false, true, true, false, false, true];

#[test]
fn the_frame_is_painted_again_exactly_when_the_guarded_presses_arm_or_disarm() {
    for (s, press) in GUARDED {
        let shown = Some(s);
        let (mut a, mut lit) = (painted(seen(Arm::default(), shown, true, T - 1), shown), false);
        for ((now, focused, acts), dirty) in LOOKS.into_iter().zip(DIRTY) {
            let was = a;
            a = seen(a, shown, focused, now);
            if now == T + ARM_MS {
                assert_eq!(a, was, "{s:?}: only the time moved, yet the look flips");
            }
            let (l, d) = relit(lit, &a, shown, now);
            assert_eq!((l, d), (acts, dirty), "{s:?} at {now}, focused {focused}");
            assert_eq!(l, !greys(&a, shown, press, now), "{s:?}: lit is what is drawn");
            lit = l;
        }
        let other = Some(if s == Shown::Export { Shown::PayReview } else { Shown::Export });
        let b = painted(a, other);
        assert_eq!(relit(lit, &b, other, T + 9 * ARM_MS), (false, true), "{s:?}: a new screen");
        assert_eq!(relit(false, &Arm::default(), None, T), (false, false), "none shows");
    }
}

#[test]
fn only_this_screens_guarded_presses_grey_and_only_on_it() {
    let pay = Some(Shown::PayReview);
    let a = seen(painted(Arm::default(), pay), pay, true, T);
    let armed = T + ARM_MS;
    assert!(shows_armed(&a, pay, armed) && pressed(a, pay, true, armed));
    assert!(!greys(&a, pay, Some(Footer(0)), armed), "armed: drawn as normal");
    assert!(greys(&a, Some(Shown::Export), Some(Footer(0)), armed), "not this screen's guard");
    assert!(!greys(&a, pay, Some(Footer(1)), T), "Edit never greys");
    assert!(!greys(&a, pay, Some(Back), T), "Back never greys");
    assert!(!greys(&Arm::default(), None, Some(Footer(0)), T), "no guarded screen shows");
}
