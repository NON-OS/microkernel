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

//! The SD pins' pull resistors (rts5227_sd_pull_ctl_enable_tbl and
//! _disable_tbl): data, CD, WP and CMD pulled up and CLK down while a card
//! is powered; everything but CD pulled down when it is not.

use crate::error::Result;
use crate::regs::card::{CARD_PULL_CTL2, CARD_PULL_CTL3};
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn pull_enable(drv: &Driver) -> Result<()> {
    set(drv, 0xAA, 0xE9)
}

pub fn pull_disable(drv: &Driver) -> Result<()> {
    set(drv, 0x55, 0xD5)
}

fn set(drv: &Driver, ctl2: u8, ctl3: u8) -> Result<()> {
    let mut buf = CmdBuf::new();
    buf.write(CARD_PULL_CTL2, 0xFF, ctl2);
    buf.write(CARD_PULL_CTL3, 0xFF, ctl3);
    crate::engine::send(drv, &buf, 100)
}
