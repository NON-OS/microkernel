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

//! A mouse that resets while the driver serves is brought up again with the
//! keyboard still typing.

use nonos_libc::{take_events, INPUT_KIND_KEY_UP};

use super::controller::{Keyboard, MouseKind};
use super::serving::{bring_ups, deliver, serving, A_MAKE, RESET};
use crate::server::reinit::reinit_mouse;

const A_BREAK: u8 = 0x9E;

#[test]
fn a_key_let_go_while_the_mouse_is_brought_up_again_is_not_lost() {
    let (ctl, _port, mut ctx) = serving(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    deliver(&ctl, &mut ctx, &[(A_MAKE, false)]);
    deliver(&ctl, &mut ctx, &RESET);
    ctl.borrow_mut().keyboard.typed = Some(A_BREAK);
    take_events();
    reinit_mouse(&mut ctx);
    assert_eq!(bring_ups(&ctl), 1, "the mouse was not brought up again");
    assert!(ctl.borrow().keyboard.typed.is_none(), "the key was never typed");
    let keys: Vec<_> = take_events().iter().map(|e| (e.kind, e.code)).collect();
    assert_eq!(keys, [(INPUT_KIND_KEY_UP, u32::from(b'a'))], "the release was dropped");
    assert!(ctx.driver.mouse_enabled);
    assert!(!ctx.mouse.take_reset(), "the reset was left pending");
}
