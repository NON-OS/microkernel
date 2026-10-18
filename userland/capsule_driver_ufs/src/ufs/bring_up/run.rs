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

//! The bring-up, stage by stage (stage.rs). The link stays in PWM-G1 on one
//! lane, where startup leaves it; a faster gear is not yet asked for.

use super::say::failed;
use super::stage::Stage;
use super::units::serve;
use crate::ufs::ctx::Ctx;
use crate::ufs::device::ready::{answers, initialised};
use crate::ufs::disk::Ufs;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::host::caps::{self, Caps};
use crate::ufs::host::{hce, link, queues};
use crate::ufs::slot::layout::Lists;

pub fn bring_up<H: Hw>(hw: &H, l: Lists, intel: bool) -> Result<Ufs, UfsError> {
    let at = |s: Stage| move |e| failed(hw, s.name(), e);
    hce::restart(hw).map_err(at(Stage::Host))?;
    let caps = caps::read(hw);
    reachable(&caps, &l).map_err(at(Stage::Dma))?;
    link::up(hw, intel).map_err(at(Stage::Link))?;
    queues::run(hw, l.utrl.bus, l.utmrl.bus).map_err(at(Stage::Queues))?;
    let ctx = Ctx::new(l, caps);
    answers(hw, &ctx).map_err(at(Stage::Answers))?;
    initialised(hw, &ctx).map_err(at(Stage::Initialised))?;
    serve(hw, ctx)
}

/// Every region below 4 GiB unless the host addresses 64 bits: refused
/// before the link, rather than DMA landing at an address cut short.
fn reachable(caps: &Caps, l: &Lists) -> Result<(), UfsError> {
    let regions = [l.utrl, l.utmrl, l.ucd, l.data];
    match caps.a64 || regions.iter().all(|r| r.bus + r.len as u64 <= 1 << 32) {
        true => Ok(()),
        false => Err(UfsError::Dma32),
    }
}
