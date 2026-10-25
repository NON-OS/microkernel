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

//! A bind's vectors: each an event of the device and an LPI, taken and given
//! back together.

use super::super::msi::Vector;
use super::encode::cmd::{discard, Cmd};
use super::lpis::{give_event, give_lpi, take_event, take_lpi};
use super::state::Its;

pub(super) fn discards<'c>(device_id: u32, vectors: &[Vector], cmds: &'c mut [Cmd]) -> &'c [Cmd] {
    let n = vectors.len().min(cmds.len());
    for (cmd, v) in cmds.iter_mut().zip(&vectors[..n]) {
        *cmd = discard(device_id, v.data);
    }
    &cmds[..n]
}

pub(super) fn take(its: &mut Its, at: usize) -> Option<Vector> {
    let dev = its.devices[at].as_mut()?;
    let event = take_event(&mut dev.events, its.event_bits)?;
    match take_lpi(&mut its.lpis) {
        Some(intid) => Some(Vector { intid, data: event }),
        None => {
            give_event(&mut dev.events, event);
            None
        }
    }
}

pub(super) fn forget(its: &mut Its, at: usize, vectors: &[Vector]) {
    for v in vectors {
        give_lpi(&mut its.lpis, v.intid);
        if let Some(dev) = its.devices[at].as_mut() {
            give_event(&mut dev.events, v.data);
        }
    }
}
