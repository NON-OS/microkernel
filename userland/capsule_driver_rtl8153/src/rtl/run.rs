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

//! Running a table of steps in order. Every wait is a bounded number of
//! looks; the first transfer that fails stops the table with its errno.

use nonos_usbnet::Bus;

use super::access::{get, put, Stop};
use super::op::{Op, Target, Wait};
use super::pause::Pause;

type Look<'a> = (Target, u32, &'a [u32], Wait);

fn until<B: Bus + Pause>(bus: &mut B, page: &mut Option<u16>, look: Look) -> Result<(), Stop> {
    let (t, mask, accept, w) = look;
    for _ in 0..w.tries {
        if accept.contains(&(get(bus, page, t)? & mask)) {
            return Ok(());
        }
        bus.pause_ms(w.every_ms);
    }
    w.fail.map_or(Ok(()), |what| Err(Stop::TimedOut(what)))
}

fn edit<B: Bus>(
    bus: &mut B,
    page: &mut Option<u16>,
    t: Target,
    off: u32,
    on: u32,
) -> Result<(), Stop> {
    let v = get(bus, page, t)?;
    put(bus, page, t, (v & !off) | on)
}

pub fn run<B: Bus + Pause>(bus: &mut B, page: &mut Option<u16>, ops: &[Op]) -> Result<(), Stop> {
    for op in ops {
        match *op {
            Op::Edit(t, off, on) => edit(bus, page, t, off, on)?,
            Op::Put(t, v) => put(bus, page, t, v)?,
            Op::Until(t, mask, accept, w) => until(bus, page, (t, mask, accept, w))?,
            Op::EditIf(test, bits, t, off, on) => {
                if get(bus, page, test)? & bits != 0 {
                    edit(bus, page, t, off, on)?;
                }
            }
            Op::Pause(ms) => bus.pause_ms(ms),
        }
    }
    Ok(())
}
