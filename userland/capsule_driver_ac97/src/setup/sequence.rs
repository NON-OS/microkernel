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

//! The bring-up: claim, command bits and port grants; the DMA grant; the
//! engine stopped in case firmware left it running; the link out of reset
//! and the codec ready at 48 kHz; outputs muted until a stream starts; the
//! list written. A failing step gives back every grant and the claim.

use nonos_libc::mk_dma_unmap;

use super::{claim, dma, driver::Driver};
use crate::codec;
use crate::constants::ring::BDL_OFFSET;
use crate::discover::Found;
use crate::engine;
use crate::regs::{Mixer, Port};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let grants = claim::claim(dev.device_id)?;
    let dma = dma::map(dev.device_id, grants.epoch).ok_or_else(|| {
        grants.release();
        "dma map failed"
    })?;
    let back = |e: &'static str| {
        let _ = mk_dma_unmap(dma.grant_id);
        grants.release();
        e
    };
    let bus = Port::new(grants.nabm);
    let mixer = Mixer::new(Port::new(grants.nam), bus);
    if !engine::stop(&bus) {
        return Err(back("engine did not stop"));
    }
    codec::bring_up(&bus).map_err(back)?;
    let ident = codec::reset(&mixer).map_err(back)?;
    codec::open(&mixer, false);
    let driver = Driver { dma, bus, mixer, ident };
    engine::fill(dma.user_va + BDL_OFFSET, driver.ring_dev());
    Ok(driver)
}
