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

//! A control read on one output's bus: allowlisted, named by the display's
//! capabilities and paced before a byte goes out, the reply checked before
//! one is believed (writes in transact_write.rs).

use super::bus::DdcBus;
use super::error::DdcError;
use super::frame::get_request;
use super::pace::{Pacer, REPLY_US};
use super::reply::{parse, Reading, REPLY_LEN};
use super::vcp::may_read;
use super::wait::{wait, wait_turn};
use crate::engine::access::Clock;

pub fn read(
    b: &mut dyn DdcBus,
    c: &dyn Clock,
    p: &mut Pacer,
    code: u8,
) -> Result<Reading, DdcError> {
    may_read(code, p.caps.caps()?)?;
    let now = c.now_us().ok_or(DdcError::NoClock)?;
    if let Some(r) = p.kept(code, now) {
        return r;
    }
    let sent = wait_turn(c, p)?;
    p.sent(sent);
    let got = exchange(b, c, code);
    p.sent(c.now_us().ok_or(DdcError::NoClock)?);
    p.keep(code, Some(got), sent);
    got
}

/// The request, the reply's wait, the reply and its checks.
fn exchange(b: &mut dyn DdcBus, c: &dyn Clock, code: u8) -> Result<Reading, DdcError> {
    b.send(&get_request(code)).map_err(|_| DdcError::Absent)?;
    wait(c, REPLY_US);
    let mut r = [0u8; REPLY_LEN];
    b.receive(&mut r)?;
    parse(code, &r)
}
