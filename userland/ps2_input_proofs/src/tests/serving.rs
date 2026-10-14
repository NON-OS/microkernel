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

//! The driver as the serve loop holds it once setup is done, over a
//! controller whose keyboard and aux port are both on.

use super::controller::{Keyboard, MouseKind};
use super::shared::{machine, Handle};
use crate::constants::{CONFIG_IRQ1, CONFIG_IRQ12, CONFIG_XLATE, CTL_DISABLE_KBD};
use crate::server::context::Context;
use crate::server::pump::pump;
use crate::setup::Driver;
use nonos_libc::Attached;

/// The configuration byte both bring-ups leave: both interrupts, both
/// clocks and translation on.
const SERVING: u8 = CONFIG_IRQ1 | CONFIG_IRQ12 | CONFIG_XLATE;
/// What a mouse sends when it resets: its self-test result and its id.
pub const RESET: [(u8, bool); 2] = [(0xAA, true), (0x00, true)];
/// The make code of the A key.
pub const A_MAKE: u8 = 0x1E;

pub fn serving(keyboard: Keyboard, mouse: MouseKind) -> (Handle, Attached, Context) {
    let (ctl, port) = machine(keyboard, mouse);
    ctl.borrow_mut().config = SERVING;
    let mouse_enabled = mouse != MouseKind::Absent;
    let driver = Driver {
        pio_grant_id: 7,
        irq_grant_id: 0,
        aux_irq_grant_id: 0,
        mouse_enabled,
        mouse_wheel: false,
    };
    (ctl, port, Context::new(driver))
}

/// Put `bytes` in the output buffer, each with its aux flag, and run one
/// pass of the serve loop's pump over them.
pub fn deliver(ctl: &Handle, ctx: &mut Context, bytes: &[(u8, bool)]) {
    ctl.borrow_mut().output.extend(bytes.iter().copied());
    pump(ctx, &mut 0);
}

/// How many times the mouse was brought up: each bring-up holds the
/// keyboard port off once, while it reads the configuration byte.
pub fn bring_ups(ctl: &Handle) -> usize {
    ctl.borrow().commands().iter().filter(|&&c| c == CTL_DISABLE_KBD).count()
}
