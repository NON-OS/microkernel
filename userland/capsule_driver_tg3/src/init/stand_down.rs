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

//! tg3_halt for a shutdown: the firmware paused and told, every engine
//! stopped, then a chip reset, so nothing the part was doing can reach
//! memory once the broker hands the grants to someone else. Linux puts the
//! station address back after the reset; this driver does not, so the
//! drawn address does not outlive it.

use crate::chip::fw::{self, Kind};
use crate::log::Line;
use crate::setup::Driver;

use super::{abort, reset};

pub fn run(d: &mut Driver) {
    let regs = d.regs;
    fw::stop(&regs, d.modes.asf);
    fw::sig_pre_reset(&regs, d.modes.asf, Kind::Shutdown);
    abort::run(d);
    if let Err(e) = reset::chip_reset(d) {
        Line::new().text("stand-down reset: ").text(e).send();
    }
    fw::sig_legacy(&regs, d.modes.asf, Kind::Shutdown);
    fw::sig_post_reset(&regs, d.modes.asf, Kind::Shutdown);
}
