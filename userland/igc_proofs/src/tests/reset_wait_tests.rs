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

//! The global reset waited out on the clock: a part that clears CTRL.RST,
//! with or without its NVM auto-read, and one that never does, which is
//! named once the bound has passed.

use std::time::Duration;

use nonos_devmodel::FakeBar;

use crate::constants::ctrl::{CTRL_RST, EECD_AUTO_RD};
use crate::constants::regs::{REG_CTRL, REG_EECD};
use crate::constants::timeouts::RESET_MS;
use crate::init::reset::global::RESET_STUCK;
use crate::init::reset::run;

use super::model::{regs, stable32, timed, window};

/// A part whose CTRL.RST clears on the next look, NVM auto-read or not.
pub fn resets(nvm: bool) -> impl Fn(&FakeBar) + Send + 'static {
    move |b| {
        let Some(ctrl) = stable32(b, REG_CTRL) else { return };
        if ctrl & CTRL_RST != 0 {
            b.present32(REG_EECD, if nvm { EECD_AUTO_RD } else { 0 });
            b.present32(REG_CTRL, ctrl & !CTRL_RST);
        }
    }
}

#[test]
fn a_part_that_never_leaves_reset_is_named_on_the_clock() {
    let bar = window();
    let (r, took) = timed(|| run(&regs(&bar)));
    assert_eq!(r, Err(RESET_STUCK));
    assert!(took >= Duration::from_millis(RESET_MS));
}
