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

//! A mouse that keeps resetting (a failing cable, an EC in a loop) is
//! brought up again three times in a minute and then turned off, once,
//! with the keyboard left working.

use nonos_libc::{mk_idle_ms, take_debug, take_events, INPUT_KIND_KEY_DOWN};

use super::controller::{Keyboard, MouseKind};
use super::serving::{bring_ups, deliver, serving, A_MAKE, RESET};
use crate::constants::{CTL_DISABLE_AUX, CTL_ENABLE_KBD};
use crate::server::reinit::reinit_mouse;

const GAVE_UP: &str = "again after three bring-ups within a minute; pointer off";

#[test]
fn a_mouse_that_keeps_resetting_is_turned_off_after_three_in_a_minute() {
    let (ctl, _port, mut ctx) = serving(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    take_debug();
    for _ in 0..6 {
        deliver(&ctl, &mut ctx, &RESET);
        reinit_mouse(&mut ctx);
    }
    assert_eq!(bring_ups(&ctl), 3, "brought up again past the cap");
    assert!(!ctx.driver.mouse_enabled, "the pointer stayed on");
    let commands = ctl.borrow().commands();
    assert!(commands.contains(&CTL_DISABLE_AUX), "the aux port was left on");
    assert_eq!(commands.last(), Some(&CTL_ENABLE_KBD), "the keyboard port was left off");
    let said = take_debug().iter().filter(|l| l.contains(GAVE_UP)).count();
    assert_eq!(said, 1, "the pointer going off was not said exactly once");
    take_events();
    deliver(&ctl, &mut ctx, &[(A_MAKE, false)]);
    let downs = take_events().iter().filter(|e| e.kind == INPUT_KIND_KEY_DOWN).count();
    assert_eq!(downs, 1, "the keyboard stopped working with the pointer");
}

#[test]
fn resets_more_than_a_minute_apart_are_each_brought_up_again() {
    let (ctl, _port, mut ctx) = serving(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    for _ in 0..5 {
        deliver(&ctl, &mut ctx, &RESET);
        reinit_mouse(&mut ctx);
        let _ = mk_idle_ms(61_000);
    }
    assert_eq!(bring_ups(&ctl), 5);
    assert!(ctx.driver.mouse_enabled);
}
