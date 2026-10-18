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

//! A controller with a codec that can play, made ready to: the stream's
//! memory mapped and silent, the codec programmed with its outputs muted.

use alloc::boxed::Box;

use super::dma;
use super::linked::{Linked, Ready};
use super::mark::{mark, Line};
use super::place::{output_descriptor, outputs, reachable};
use super::ring::{position_buffer, silence};
use super::Driver;
use crate::constants::STREAM_TAG;
use crate::controller::codec::{plan::Plan, program::program, widget::Codec};
use crate::controller::verdict::Verdict;
use crate::discover::Found;
use crate::error::HdaResult;
use crate::protocol::OutputStatus;

pub(super) fn play(dev: Found, mut l: Linked, codec: Box<Codec>, plan: Plan) -> HdaResult<Driver> {
    for o in plan.iter() {
        let mut line = Line::new("[HDA] path cad=");
        line.dec(codec.cad as u32).s(" pin=").hex(o.pin() as u32, 2).s(" dac=");
        line.hex(o.path.dac() as u32, 2).s(" hops=").dec(o.path.len as u32).emit();
    }
    let out = output_descriptor(l.info)?;
    let (bdl, sample) = dma::map_stream(dev.device_id, l.epoch)?;
    reachable(l.info.addr64, &[&bdl, &sample])?;
    silence(&sample);
    let plugged = program(&mut l.link, &codec, &plan, STREAM_TAG)?;
    let posbuf_va = position_buffer(l.regs, &dev, &bdl, out.global_index);
    mark("[HDA] ready: outputs muted, no stream until a player opens one\n");
    let status = OutputStatus {
        verdict: Verdict::Ready as u32,
        codec_vendor: (codec.vendor_id >> 16) as u16,
        codec_device: codec.vendor_id as u16,
        outputs: outputs(&plan),
        plugged,
    };
    Ok(l.into_driver(Ready { codec, plan, bdl, sample, out, posbuf_va, status }))
}
