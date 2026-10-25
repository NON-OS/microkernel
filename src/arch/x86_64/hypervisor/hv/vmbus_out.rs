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

/*
 * What the VMBus core may say to the host. Every message it posts is held
 * here before the hypercall: one of the channel messages a guest sends,
 * long enough for its fields, aimed at VP 0 where the SynIC is, and every
 * guest page it names held as DMA. The contact's monitor and interrupt
 * pages are the core's own, on the VMBus root. A GPADL's frames are the
 * channel's, held on the record of the channel it is for. That channel must
 * be one the host offered and the core gave a driver (`channels`), and an
 * open must use that channel's own GPADL. Anything else would let a
 * capsule hand the host memory that is not its own, or open a host service
 * no driver was given.
 */

use super::channels::Channels;

pub(super) const MESSAGE_TYPE_CHANNEL: u32 = 1;
/// The connections the host takes channel messages on: 1 before VMBus 5.0,
/// 4 from it.
pub(super) const CONNECTION_MESSAGE: u32 = 1;
pub(super) const CONNECTION_MESSAGE_V5: u32 = 4;

const REQUESTOFFERS: u32 = 3;
const OPENCHANNEL: u32 = 5;
const CLOSECHANNEL: u32 = 7;
const GPADL_HEADER: u32 = 8;
const GPADL_BODY: u32 = 9;
const GPADL_TEARDOWN: u32 = 11;
const RELID_RELEASED: u32 = 13;
const INITIATE_CONTACT: u32 = 14;
const UNLOAD: u32 = 16;

const VERSION_V5: u32 = 5 << 16;
const PAGE: u64 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// Not a channel message on a channel connection.
    Route,
    /// A channel message a guest does not send, or one the core may not.
    Kind,
    /// Shorter than its fields.
    Short,
    /// Aimed at a VP other than 0, or a SINT other than VMBus's.
    Target,
    /// A page the caller does not hold.
    Page,
    /// A GPADL or open for no channel given to a driver, or an open whose
    /// GPADL is not that channel's.
    Channel,
}

/// Who must hold a page a message names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holder {
    /// The caller, on the VMBus root.
    Root,
    /// Anyone, on this channel's record.
    Record(u64),
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(p: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(p.get(at..at + 8)?.try_into().ok()?))
}

/// Every frame from `at` to the end of `p`, eight bytes each.
fn frames(p: &[u8], at: usize, held: &dyn Fn(u64) -> bool) -> Result<(), Refusal> {
    let list = p.get(at..).ok_or(Refusal::Short)?;
    if list.is_empty() || list.len() % 8 != 0 {
        return Err(Refusal::Short);
    }
    for f in list.chunks_exact(8) {
        let pfn = u64::from_le_bytes(f.try_into().map_err(|_| Refusal::Short)?);
        let gpa = pfn.checked_mul(PAGE).ok_or(Refusal::Page)?;
        if !held(gpa) {
            return Err(Refusal::Page);
        }
    }
    Ok(())
}

/// Whether `payload`, posted as `ty` on `connection`, may go to the host.
/// `held` says whether the page at a guest address is held by `Holder`.
pub(super) fn check(
    connection: u32,
    ty: u32,
    payload: &[u8],
    channels: &Channels,
    held: impl Fn(Holder, u64) -> bool,
) -> Result<(), Refusal> {
    let root = |gpa: u64| held(Holder::Root, gpa);
    if ty != MESSAGE_TYPE_CHANNEL
        || !matches!(connection, CONNECTION_MESSAGE | CONNECTION_MESSAGE_V5)
    {
        return Err(Refusal::Route);
    }
    let kind = u32_at(payload, 0).ok_or(Refusal::Short)?;
    let need = |n: usize| if payload.len() < n { Err(Refusal::Short) } else { Ok(()) };
    match kind {
        REQUESTOFFERS | UNLOAD => need(8),
        CLOSECHANNEL | RELID_RELEASED => need(12),
        GPADL_TEARDOWN => need(16),
        OPENCHANNEL => {
            need(28)?;
            if u32_at(payload, 20) != Some(0) {
                return Err(Refusal::Target);
            }
            channels.target(payload).map(|_| ()).ok_or(Refusal::Channel)
        }
        INITIATE_CONTACT => {
            need(40)?;
            let version = u32_at(payload, 8).ok_or(Refusal::Short)?;
            if u32_at(payload, 12) != Some(0) {
                return Err(Refusal::Target);
            }
            // From 5.0 the third word is the message SINT; before, a page.
            if version >= VERSION_V5 {
                if payload[16] != super::regs::SINT_VMBUS as u8 {
                    return Err(Refusal::Target);
                }
            } else if !root(u64_at(payload, 16).ok_or(Refusal::Short)?) {
                return Err(Refusal::Page);
            }
            for at in [24, 32] {
                if !root(u64_at(payload, at).ok_or(Refusal::Short)?) {
                    return Err(Refusal::Page);
                }
            }
            Ok(())
        }
        GPADL_HEADER => {
            // One range, its byte count and offset, then its first frames.
            need(28)?;
            if u32_at(payload, 16).map(|w| w >> 16) != Some(1) {
                return Err(Refusal::Kind);
            }
            let record = channels.target(payload).ok_or(Refusal::Channel)?;
            if !channels.gpadl_free(u32_at(payload, 12).ok_or(Refusal::Short)?) {
                return Err(Refusal::Channel);
            }
            frames(payload, 28, &|gpa| held(Holder::Record(record), gpa))
        }
        GPADL_BODY => {
            need(16)?;
            let record = channels.target(payload).ok_or(Refusal::Channel)?;
            frames(payload, 16, &|gpa| held(Holder::Record(record), gpa))
        }
        _ => Err(Refusal::Kind),
    }
}
