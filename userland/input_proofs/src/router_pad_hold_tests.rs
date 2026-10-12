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

//! Under a prompt or the lock screen the pad is the trusted band's holder's
//! alone: no press, release or axis reaches the focused window under it.

use nonos_libc::mk_uptime_ms;

use crate::route::pad::{PAD_AXIS, PAD_BUTTON_DOWN, PAD_BUTTON_UP};
use crate::route::trusted_hold::told;
use crate::router_desk::*;
use crate::router_pad_tests::{pad, two_windows, SOUTH};

#[test]
fn under_a_prompt_the_pad_is_the_holders() {
    let mut ctx = two_windows();
    ctx.trusted = told(SHELL, mk_uptime_ms());
    pad(&mut ctx, PAD_BUTTON_DOWN, 0, SOUTH);
    pad(&mut ctx, PAD_AXIS, 0, 1);
    pad(&mut ctx, PAD_BUTTON_UP, 0, SOUTH);
    let rx = sent();
    assert_eq!(rx.len(), 3);
    assert!(rx.iter().all(|r| r.pid == SHELL), "a window under the prompt read the pad: {rx:?}");
}
