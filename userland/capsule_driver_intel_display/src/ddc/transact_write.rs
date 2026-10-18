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

//! A control written on one output's bus: allowed and named as a read is,
//! the value held to the maximum the monitor gave or to what it listed,
//! and the kept reading forgotten so the next read asks again.

use super::bus::DdcBus;
use super::error::DdcError;
use super::frame::set_request;
use super::pace::Pacer;
use super::vcp::{may_read, may_set, BRIGHTNESS, CONTRAST};
use super::wait::wait_turn;
use crate::engine::access::Clock;

pub fn write(
    b: &mut dyn DdcBus,
    c: &dyn Clock,
    p: &mut Pacer,
    code: u8,
    value: u16,
) -> Result<(), DdcError> {
    let caps = *p.caps.caps()?;
    may_read(code, &caps)?;
    let max = match code {
        BRIGHTNESS | CONTRAST => super::transact::read(b, c, p, code)?.max,
        _ => 0,
    };
    may_set(code, value, max, &caps)?;
    let at = wait_turn(c, p)?;
    p.keep(code, None, at);
    p.sent(at);
    b.send(&set_request(code, value)).map_err(|_| DdcError::Absent)
}
