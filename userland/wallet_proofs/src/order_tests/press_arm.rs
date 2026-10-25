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

//! A guarded screen off the purchase arms as Buy does, by the same
//! arming: drawn, focused, held a second, asked again at the press. Focus
//! lost disarms it (no terms to read again); a fresh second arms it.

use crate::order::terms::arming::{Guard, ARM_MS};
use crate::wallet::arm::look::{painted, pressed, seen, Arm};
use crate::wallet::arm::moves::Shown;

const T: u64 = 70_000;
const PAY: Option<Shown> = Some(Shown::PayReview);

/// PAY drawn, then focused at `t`.
fn held_from(t: u64) -> Arm {
    seen(painted(Arm::default(), PAY), PAY, true, t)
}

#[test]
fn a_guarded_screen_arms_one_second_after_it_is_drawn_and_focused() {
    let undrawn = seen(Arm::default(), PAY, true, T);
    assert!(!pressed(seen(undrawn, PAY, true, T + 1), PAY, true, T + 5 * ARM_MS), "not drawn");
    let a = held_from(T);
    assert_eq!(a.guard, Guard { drawn: true, since: Some(T) });
    assert!(!pressed(a, PAY, true, T + ARM_MS - 1));
    assert!(pressed(a, PAY, true, T + ARM_MS));
    assert!(!pressed(a, PAY, false, T + ARM_MS), "focus asked again at the press");
    assert!(!pressed(a, Some(Shown::Meter), true, T + ARM_MS), "not another screen's press");
    assert!(!pressed(a, None, true, T + ARM_MS));
}

#[test]
fn focus_lost_off_the_purchase_disarms_until_a_fresh_second() {
    let lost = seen(held_from(T), PAY, false, T + 2 * ARM_MS);
    let disarmed = Arm { shown: PAY, guard: Guard { drawn: true, since: None } };
    assert_eq!(lost, disarmed, "still drawn, nothing read again, unarmed");
    assert!(!pressed(lost, PAY, true, T + 2 * ARM_MS));
    let back = T + 3 * ARM_MS;
    let regained = seen(lost, PAY, true, back);
    assert!(!pressed(regained, PAY, true, back + ARM_MS - 1), "the old second is gone");
    assert!(pressed(regained, PAY, true, back + ARM_MS));
}

#[test]
fn another_screen_starts_the_guard_afresh() {
    let a = held_from(T);
    assert_eq!(painted(a, PAY), a, "the same screen painted again keeps its second");
    let stake = Some(Shown::StakeReview);
    let b = painted(a, stake);
    assert_eq!(b, Arm { shown: stake, guard: Guard { drawn: true, since: None } });
    assert!(!pressed(b, stake, true, T + 10 * ARM_MS));
    assert_eq!(seen(a, stake, true, T + 1), Arm { shown: stake, guard: Guard::default() });
    assert_eq!(painted(a, None), Arm::default());
    assert_eq!(seen(a, None, true, T + 1), Arm::default());
}
