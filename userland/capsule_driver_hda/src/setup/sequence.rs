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
use core::ptr::write_bytes;

use nonos_libc::{mk_device_release, DmaMapOut};

use super::choose::choose;
use super::driver::Started;
use super::mark::{mark, Line};
use super::{claim, dma, irq, mmio, pci};
use crate::constants::{DPLBASE, DPLBASE_ENABLE, DPUBASE, GCAP, STREAM_TAG};
use crate::controller::bdl::{POSBUF_OFFSET, RING_BYTES};
use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::program::program;
use crate::controller::verb::Link;
use crate::controller::verdict::{combine, judge, Verdict};
use crate::controller::{corb, intel, layout, probe, reset};
use crate::controller::{ControllerInfo, StreamDescriptor, STREAM_BIDI, STREAM_OUTPUT};
use crate::discover::{Found, Survey};
use crate::error::{HdaError, HdaResult};
use crate::handles::BrokerHandles;
use crate::protocol::{OutputStatus, OUT_HEADPHONE, OUT_LINE, OUT_SPEAKER};
use crate::regs::Regs;
use crate::setup::Driver;

const FOUR_GIB: u64 = 1 << 32;

enum Attempt {
    Playing(Driver),
    Silent(Verdict),
}

/// One bring-up attempt over every controller discovery found, chipset
/// controllers first. The first that can play is kept; the others' claims
/// are given back as each attempt ends. A graphics card's controller that
/// fails is passed over, since it never carries the speakers; any other
/// failure fails the attempt, so the bounded retry schedule runs again.
pub fn run(survey: &Survey, verdict_seen: &mut Option<Verdict>) -> HdaResult<Started> {
    let mut last_err = None;
    for dev in survey.controllers() {
        match one(*dev) {
            Ok(Attempt::Playing(d)) => return Ok(Started::Playing(d)),
            Ok(Attempt::Silent(v)) => {
                *verdict_seen = Some(verdict_seen.map_or(v, |w| combine(w, v)));
            }
            Err(_) if dev.graphics() => {}
            Err(e) => last_err = Some(e),
        }
    }
    if let Some(e) = last_err {
        return Err(e);
    }
    Ok(Started::Silent(OutputStatus::silent(machine_verdict(survey, *verdict_seen) as u32)))
}

/// What the machine as a whole is, once no controller plays.
pub fn machine_verdict(survey: &Survey, seen: Option<Verdict>) -> Verdict {
    let v = seen.unwrap_or(Verdict::NoCodec);
    if survey.amd_acp {
        combine(v, Verdict::AmdAcp)
    } else {
        v
    }
}

fn one(dev: Found) -> HdaResult<Attempt> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let quirks = intel::pci_quirks(dev.vendor, dev.device);
    pci::prepare(dev.device_id, claim_epoch, quirks);
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch)?;
    let (corb, rirb) = dma::map_verb_rings(dev.device_id, claim_epoch, &mmio, &irq)?;
    let handles = BrokerHandles::new(
        dev.device_id,
        mmio.grant_id,
        mmio.user_va,
        irq.grant_id,
        corb.grant_id,
        rirb.grant_id,
    );
    let regs = Regs::new(handles.mmio_user_va());
    Line::new("[HDA] controller ").hex(dev.vendor as u32, 4).s(":").hex(dev.device as u32, 4).emit();
    if unsafe { regs.r16(GCAP) } == 0xffff {
        return Err(HdaError::ControllerNotResponding);
    }
    pci::clock_gating(dev.device_id, claim_epoch, quirks, false);
    let reset = reset::reset_link(regs);
    pci::clock_gating(dev.device_id, claim_epoch, quirks, true);
    let codec_mask = reset?;
    intel::reduce_dma_latency(regs, dev.vendor, dev.device, dev.bar_size);
    intel::init_link_clock(regs, dev.vendor, dev.device, dev.bar_size);
    let info = ControllerInfo::read(regs);
    if info.vmaj == 0 || info.gcap == 0 {
        return Err(HdaError::UnsupportedController);
    }
    reachable(info.addr64, &[&corb, &rirb])?;
    let rings = corb::init(regs, corb.device_addr, rirb.device_addr);
    if !rings.rp_handshake {
        mark("[HDA] corb read pointer reset not acknowledged; continuing as Linux does\n");
    }
    let mut link = Link::new(regs, corb.user_va, rirb.user_va, rings.entries);
    Line::new("[HDA] codecs mask=").hex(codec_mask as u32, 4).emit();
    let codecs = probe(&mut link, codec_mask);
    let choice = choose(&mut link, &codecs, codec_mask);
    let dsp = intel::dsp_capable(dev.vendor, dev.device, dev.subclass);
    let Some((codec, plan)) = choice.best else {
        let v = judge(dsp, choice.findings);
        Line::new("[HDA] no analog output on this controller, verdict ").dec(v as u32).emit();
        return Ok(Attempt::Silent(v));
    };
    for o in plan.iter() {
        Line::new("[HDA] path cad=").dec(codec.cad as u32).s(" pin=").hex(o.pin() as u32, 2)
            .s(" dac=").hex(o.path.dac() as u32, 2).s(" hops=").dec(o.path.len as u32).emit();
    }
    let out = output_descriptor(info)?;
    let prior = [corb.grant_id, rirb.grant_id];
    let (bdl, sample) = dma::map_stream(dev.device_id, claim_epoch, &mmio, &irq, &prior)?;
    reachable(info.addr64, &[&bdl, &sample])?;
    // The ring starts silent and the engine stays stopped: the first sound
    // is the first period a player writes after OP_STREAM_START. A ring
    // filled at init and left running loops forever, and the codec's outputs
    // stay muted until then too (`program`).
    unsafe { write_bytes(sample.user_va as *mut u8, 0, RING_BYTES as usize) };
    crate::controller::dma_sync::flush(sample.user_va, RING_BYTES);
    let plugged = program(&mut link, &codec, &plan, STREAM_TAG)?;
    let posbuf_va = position_buffer(regs, &dev, &bdl, out.global_index);
    mark("[HDA] ready: outputs muted, no stream until a player opens one\n");
    let status = OutputStatus {
        verdict: Verdict::Ready as u32,
        codec_vendor: (codec.vendor_id >> 16) as u16,
        codec_device: codec.vendor_id as u16,
        outputs: outputs(&plan),
        plugged,
    };
    Ok(Attempt::Playing(Driver {
        handles,
        regs,
        link,
        codec_mask,
        codecs,
        codec,
        plan,
        bdl,
        sample,
        stream_off: out.mmio_offset,
        stream_tag: STREAM_TAG,
        stream_gi: out.global_index,
        stream_kind: out.kind,
        posbuf_va,
        status,
    }))
}

fn outputs(plan: &crate::controller::codec::plan::Plan) -> u8 {
    let mut b = 0u8;
    if plan.has(OutKind::Speaker) {
        b |= OUT_SPEAKER;
    }
    if plan.has(OutKind::Headphone) {
        b |= OUT_HEADPHONE;
    }
    if plan.has(OutKind::LineOut) {
        b |= OUT_LINE;
    }
    b
}

/// A controller without GCAP.64OK drives only 32 address bits; a buffer
/// above 4 GiB would be fetched from its low half, someone else's memory.
fn reachable(addr64: u8, maps: &[&DmaMapOut]) -> HdaResult<()> {
    if addr64 != 0 {
        return Ok(());
    }
    if maps.iter().any(|m| m.device_addr.saturating_add(m.length) > FOUR_GIB) {
        return Err(HdaError::DmaOutOfReach);
    }
    Ok(())
}

/// Point DPLBASE at the position buffer in the BDL page, on the controllers
/// whose playback position is read from it, and return this stream's entry.
fn position_buffer(regs: Regs, dev: &Found, bdl: &DmaMapOut, gi: u16) -> Option<u64> {
    if !intel::position_buffer(dev.vendor) {
        return None;
    }
    let va = bdl.user_va + POSBUF_OFFSET;
    let pa = bdl.device_addr + POSBUF_OFFSET;
    unsafe {
        write_bytes(va as *mut u8, 0, 256);
        regs.w32(DPUBASE, (pa >> 32) as u32);
        regs.w32(DPLBASE, pa as u32 | DPLBASE_ENABLE);
    }
    crate::controller::dma_sync::flush(va, 256);
    Some(va + gi as u64 * 8)
}

fn output_descriptor(info: ControllerInfo) -> HdaResult<StreamDescriptor> {
    let (descs, n) = layout(info);
    let mut i = 0usize;
    while i < n {
        if descs[i].kind == STREAM_OUTPUT {
            return Ok(descs[i]);
        }
        i += 1;
    }
    // A controller with no output engine plays through a bidirectional one.
    descs[..n].iter().find(|d| d.kind == STREAM_BIDI).copied().ok_or(HdaError::UnsupportedController)
}
