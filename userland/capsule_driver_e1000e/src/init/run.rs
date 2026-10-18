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

//! The bring-up: the part stopped and reset with Bus Master off
//! (`reset_part`), the station address drawn, then `start`.

use crate::log::Line;
use crate::setup::Driver;

use super::{reset_part, start, station_address};

pub fn bring_up(d: &mut Driver) -> Result<(), &'static str> {
    reset_part::run(d)?;
    /*
     * Drawn, not read out of the NVM. The factory address identifies this
     * card to every network it ever joins, which outlives a system that keeps
     * nothing on disk.
     */
    d.mac = station_address::draw()?;
    start::run(d)
}

/// A step Linux logs and goes on from.
pub(super) fn warn(e: Option<&'static str>) {
    if let Some(e) = e {
        Line::new().text(e).text(", continuing").send();
    }
}
