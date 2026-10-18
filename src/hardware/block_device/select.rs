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

//! Which disk the block layer reads and writes: the one NONOS lives on.
//!
//! A machine may carry the firmware's boot disk on AHCI next to the data disk
//! on NVMe, or an installer's stick next to the disk it installed. A disk that
//! merely answers is not the one to write, so each backend is asked in turn
//! whether its disk carries the store header or the disk plan. With two such
//! disks, the one whose table holds the partition the loader came from is
//! kept; with no such partition named, the first, as before. Nothing is kept
//! while a driver is still starting or a caller lacks the rights to ask.

use super::backend::Backend;
use super::booted;
use super::found::Found;
use super::identify::identify;
use super::kept::{chosen, keep, settle};
use super::order::order;
use super::pass::Pass;
use super::said::say;
use super::seen_line::line;
use super::wait::refused_now;
use super::wire::endpoint_of;
use super::BlockDeviceError;

pub fn selected() -> Result<Backend, BlockDeviceError> {
    if let Some(b) = chosen() {
        return Ok(b);
    }
    let boot = booted::partition();
    let (mut other, mut refused, mut pass) = (None, None, Pass::default());
    for backend in order() {
        if pass.skips(endpoint_of(&backend)) {
            continue;
        }
        let (found, seen) = identify(backend, boot.as_ref());
        say(backend, &line(backend, &seen));
        match found {
            Found::Layout { places, boot: true } => return Ok(keep(backend, places)),
            Found::Layout { places, .. } if boot.is_none() => return Ok(keep(backend, places)),
            Found::Layout { places, .. } => {
                other.get_or_insert((backend, places));
            }
            Found::Absent => {}
            /*
             * A backend that could not be asked is not passed over: the disk
             * behind it may be the right one, and settling on another now
             * would split reads and writes across two disks.
             */
            Found::Refused(e) => {
                refused_now();
                if other.is_none() {
                    return Err(e);
                }
                pass.refused(endpoint_of(&backend), e);
                refused = Some(e);
            }
        }
    }
    settle(other, refused)
}
