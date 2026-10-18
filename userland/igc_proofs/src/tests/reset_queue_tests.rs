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

//! Queue 0 enabled by the reset is the part's default and is turned off; an
//! engine the reset left running ends the attempt by name. A queue that
//! will not turn off, or turns off late, is in reset_sticky_tests: the part
//! there acts on the driver's looks at the clock, between its write of 0
//! and its first read, which a part on its own thread cannot be timed to.

use nonos_devmodel::FakeBar;

use crate::constants::ctrl::{CTRL_RST, EECD_AUTO_RD};
use crate::constants::regs::*;
use crate::constants::rx_bits::{RCTL_EN, RXDCTL_QUEUE_ENABLE};
use crate::constants::tx_bits::TXDCTL_QUEUE_ENABLE;
use crate::init::reset::run;

use super::model::{live, regs, stable32, window};

pub const STILL_ON: Result<(), &str> = Err("a queue or engine still on after the reset");

/// Each queue's control register and enable bit.
pub const QUEUES: [(usize, u32); 2] =
    [(REG_RXDCTL, RXDCTL_QUEUE_ENABLE), (REG_TXDCTL, TXDCTL_QUEUE_ENABLE)];

/// A part whose reset ends on the next look with queue 0 enabled, as a reset
/// may leave it; `after` runs on every other look.
pub fn resets_on(after: fn(&FakeBar)) -> impl Fn(&FakeBar) + Send + 'static {
    move |b| {
        let Some(ctrl) = stable32(b, REG_CTRL) else { return };
        if ctrl & CTRL_RST == 0 {
            return after(b);
        }
        for (queue, enable) in QUEUES {
            b.present32(queue, enable);
        }
        b.present32(REG_EECD, EECD_AUTO_RD);
        b.present32(REG_CTRL, ctrl & !CTRL_RST);
    }
}

/// That reset is over, and the driver has written both queues off since.
pub fn written_off(b: &FakeBar) -> bool {
    b.wrote32(REG_EECD) & EECD_AUTO_RD != 0 && QUEUES.iter().all(|&(q, _)| b.wrote32(q) == 0)
}

#[test]
fn queue_zero_left_enabled_by_the_reset_is_turned_off() {
    let bar = window();
    let _part = live(&bar, resets_on(|_| {}));
    assert_eq!(run(&regs(&bar)), Ok(()));
    assert_eq!(bar.wrote32(REG_RXDCTL) & RXDCTL_QUEUE_ENABLE, 0, "rx queue off");
    assert_eq!(bar.wrote32(REG_TXDCTL) & TXDCTL_QUEUE_ENABLE, 0, "tx queue off");
}

#[test]
fn a_receiver_the_reset_left_on_is_named() {
    let bar = window();
    let _part = live(&bar, resets_on(|b| b.present32(REG_RCTL, RCTL_EN)));
    assert_eq!(run(&regs(&bar)), STILL_ON);
}
