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

//! One bring-up attempt on one controller, from its claim to a codec that
//! plays or the plain reason none does.

use alloc::boxed::Box;

use super::choose::choose;
use super::config::enable_bus_master;
use super::dma;
use super::linked::Linked;
use super::mark::{mark, Line};
use super::place::reachable;
use super::play::play;
use super::take::{link_up, take};
use super::Driver;
use crate::controller::verb::Link;
use crate::controller::verdict::{judge, Verdict};
use crate::controller::{corb, intel, probe};
use crate::discover::Found;
use crate::error::HdaResult;
use crate::regs::Regs;

pub(super) enum Attempt {
    Playing(Box<Driver>),
    Silent(Verdict),
}

/// Bring up `dev`. Bus mastering comes on only once the reset has stopped
/// every engine a previous owner left running, just before the rings start.
/// Every return after `take` drops the broker handles, unless they go on
/// into the `Driver`, and the drop stops the controller before anything is
/// given back: the silent verdict here, which is the normal end for a
/// graphics card's controller, as much as any error.
pub(super) fn one(dev: Found) -> HdaResult<Attempt> {
    let (handles, epoch) = take(dev)?;
    let (corb, rirb) = dma::map_verb_rings(dev.device_id, epoch)?;
    let regs = Regs::new(handles.mmio_user_va());
    let (info, codec_mask) = link_up(dev, epoch, regs)?;
    reachable(info.addr64, &[&corb, &rirb])?;
    enable_bus_master(dev.device_id, epoch)?;
    let rings = corb::init(regs, corb.device_addr, rirb.device_addr);
    if !rings.rp_handshake {
        mark("[HDA] corb read pointer reset not acknowledged; continuing as Linux does\n");
    }
    let mut link = Link::new(regs, corb.user_va, rirb.user_va, rings.entries);
    Line::new("[HDA] codecs mask=").hex(codec_mask as u32, 4).emit();
    let codecs = probe(&mut link, codec_mask);
    let choice = choose(&mut link, &codecs, codec_mask);
    let Some((codec, plan)) = choice.best else {
        let dsp = intel::dsp_capable(dev.vendor, dev.device, dev.subclass);
        let v = judge(dsp, choice.findings);
        Line::new("[HDA] no analog output on this controller, verdict ").dec(v as u32).emit();
        return Ok(Attempt::Silent(v));
    };
    let up = Linked { handles, epoch, regs, info, link, codec_mask, codecs };
    play(dev, up, codec, plan).map(|d| Attempt::Playing(Box::new(d)))
}
