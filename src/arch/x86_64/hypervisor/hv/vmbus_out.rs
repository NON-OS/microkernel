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
 * What may be said to the host, held here before the hypercall. Every
 * message is a channel message (SynIC type 1) a guest sends, long enough for
 * its fields, aimed at VP 0 where the SynIC is, and every guest page it
 * names held as DMA where it must be. Anything else would let a capsule
 * hand the host memory that is not its own, or reach a channel that is not
 * its own.
 *
 * - The VMBus core, on the root (`check`): the bus's messages alone. These
 *   are the contact, the request for offers, a withdrawn relid released and
 *   the unload. The contact's pages are the core's own, held on the root.
 * - A channel's driver, on its record (`check_channel`): its own channel's
 *   messages alone (`Channels::target`). These are the open, the close and
 *   its GPADLs, whose frames must be held on its record. The core never
 *   reaches a channel.
 */

use super::channels::{Channels, TargetError};

pub(super) const MESSAGE_TYPE_CHANNEL: u32 = 1;
/// The connections the host takes the core's messages on: 1 before VMBus
/// 5.0, 4 from it.
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
    /// A channel message a guest does not send, or one this caller may not.
    Kind,
    /// Shorter than its fields.
    Short,
    /// Aimed at a VP other than 0, or a SINT other than VMBus's.
    Target,
    /// A page not held where it must be.
    Page,
    /// Not for the caller's own channel, or a GPADL it may not make now.
    Channel(TargetError),
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(p: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(p.get(at..at + 8)?.try_into().ok()?))
}

fn need(p: &[u8], n: usize) -> Result<(), Refusal> {
    if p.len() < n {
        Err(Refusal::Short)
    } else {
        Ok(())
    }
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

/// Whether the VMBus core may post `payload` as `ty` on `connection`.
/// `held` says whether the core holds the page at a guest address on the
/// root.
pub(super) fn check(
    connection: u32,
    ty: u32,
    payload: &[u8],
    held: impl Fn(u64) -> bool,
) -> Result<(), Refusal> {
    if ty != MESSAGE_TYPE_CHANNEL
        || !matches!(connection, CONNECTION_MESSAGE | CONNECTION_MESSAGE_V5)
    {
        return Err(Refusal::Route);
    }
    let kind = u32_at(payload, 0).ok_or(Refusal::Short)?;
    match kind {
        REQUESTOFFERS | UNLOAD => need(payload, 8),
        RELID_RELEASED => need(payload, 12),
        INITIATE_CONTACT => {
            need(payload, 40)?;
            let version = u32_at(payload, 8).ok_or(Refusal::Short)?;
            if u32_at(payload, 12) != Some(0) {
                return Err(Refusal::Target);
            }
            // From 5.0 the third word is the message SINT; before, a page.
            if version >= VERSION_V5 {
                if payload[16] != super::regs::SINT_VMBUS as u8 {
                    return Err(Refusal::Target);
                }
            } else if !held(u64_at(payload, 16).ok_or(Refusal::Short)?) {
                return Err(Refusal::Page);
            }
            for at in [24, 32] {
                if !held(u64_at(payload, at).ok_or(Refusal::Short)?) {
                    return Err(Refusal::Page);
                }
            }
            Ok(())
        }
        _ => Err(Refusal::Kind),
    }
}

/// Whether a channel's driver, holding `record`, may post `payload` as
/// `ty`: one of its own channel's messages. `held` says whether a page at a
/// guest address is held on its record.
pub(super) fn check_channel(
    ty: u32,
    payload: &[u8],
    channels: &Channels,
    record: u64,
    held: impl Fn(u64) -> bool,
) -> Result<(), Refusal> {
    if ty != MESSAGE_TYPE_CHANNEL {
        return Err(Refusal::Route);
    }
    let kind = u32_at(payload, 0).ok_or(Refusal::Short)?;
    match kind {
        CLOSECHANNEL => need(payload, 12)?,
        OPENCHANNEL => need(payload, 28)?,
        GPADL_HEADER => need(payload, 28)?,
        GPADL_BODY | GPADL_TEARDOWN => need(payload, 16)?,
        _ => return Err(Refusal::Kind),
    }
    channels.target(record, payload).map_err(Refusal::Channel)?;
    match kind {
        OPENCHANNEL if u32_at(payload, 20) != Some(0) => Err(Refusal::Target),
        GPADL_HEADER => {
            // One range: its byte count and offset, then its first frames.
            if u32_at(payload, 16).map(|w| w >> 16) != Some(1) {
                return Err(Refusal::Kind);
            }
            frames(payload, 28, &held)
        }
        GPADL_BODY => frames(payload, 16, &held),
        _ => Ok(()),
    }
}
