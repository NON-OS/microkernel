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
 * The VMBus channels as the kernel knows them: learnt from the host's own
 * offers as they pass to the core, never from the core. A channel's
 * connection id and interface come from its offer. The core may give a
 * channel to a driver only by naming a relid the host offered and has not
 * withdrawn. A GPADL is tied to the channel its header named, so its body
 * frames, which name no channel, are held to the same one. Pure: kernel
 * proofs run it.
 */

/// Channels kept at once; the core keeps as many.
pub(super) const MAX_CHANNELS: usize = 64;
/// GPADLs being built or held at once.
pub(super) const MAX_GPADLS: usize = 64;
/// The relids an event flag page can signal: 2048 bits.
pub(super) const MAX_RELID: u32 = 2047;

const OFFERCHANNEL: u32 = 1;
const RESCIND_CHANNELOFFER: u32 = 2;
const OPENCHANNEL: u32 = 5;
const GPADL_HEADER: u32 = 8;
const GPADL_BODY: u32 = 9;
const GPADL_TORNDOWN: u32 = 12;
const UNLOAD_RESPONSE: u32 = 17;

const OFFER_BYTES: usize = 196;

/// The interfaces a driver may be given, and the device id its record
/// carries. Anything else (heartbeat, shutdown, time sync, key-value
/// exchange, VSS, dynamic memory, file copy, PCI) is never given out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    Network,
    Scsi,
    Ide,
    Keyboard,
    Mouse,
    Video,
}

const fn guid(a: u32, b: u16, c: u16, d: [u8; 8]) -> [u8; 16] {
    let a = a.to_le_bytes();
    let b = b.to_le_bytes();
    let c = c.to_le_bytes();
    [a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7]]
}

/// Linux's HV_*_GUID, as VMBus carries them (the first three fields little
/// endian).
const INTERFACES: [([u8; 16], Interface); 6] = [
    (
        guid(0xf861_5163, 0xdf3e, 0x46c5, [0x91, 0x3f, 0xf2, 0xd2, 0xf9, 0x65, 0xed, 0x0e]),
        Interface::Network,
    ),
    (
        guid(0xba61_63d9, 0x04a1, 0x4d29, [0xb6, 0x05, 0x72, 0xe2, 0xff, 0xb1, 0xdc, 0x7f]),
        Interface::Scsi,
    ),
    (
        guid(0x3241_2632, 0x86cb, 0x44a2, [0x9b, 0x5c, 0x50, 0xd1, 0x41, 0x73, 0x54, 0xf5]),
        Interface::Ide,
    ),
    (
        guid(0xf912_ad6d, 0x2b17, 0x48ea, [0xbd, 0x65, 0xf9, 0x27, 0xa6, 0x1c, 0x76, 0x84]),
        Interface::Keyboard,
    ),
    (
        guid(0xcfa8_b69e, 0x5b4a, 0x4cc0, [0xb9, 0x8b, 0x8b, 0xa1, 0xa1, 0xf3, 0xf9, 0x5a]),
        Interface::Mouse,
    ),
    (
        guid(0xda0a_7802, 0xe377, 0x4aac, [0x8e, 0x77, 0x05, 0x58, 0xeb, 0x10, 0x73, 0xf8]),
        Interface::Video,
    ),
];

impl Interface {
    pub fn of(guid: &[u8; 16]) -> Option<Interface> {
        INTERFACES.iter().find(|(g, _)| g == guid).map(|(_, i)| *i)
    }

    /// The device id of a channel record, two letters for the bus to read.
    pub const fn device(self) -> u16 {
        match self {
            Interface::Network => 0x4E45,  // "NE"
            Interface::Scsi => 0x5343,     // "SC"
            Interface::Ide => 0x4944,      // "ID"
            Interface::Keyboard => 0x4B42, // "KB"
            Interface::Mouse => 0x4D53,    // "MS"
            Interface::Video => 0x5644,    // "VD"
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Channel {
    pub relid: u32,
    pub connection: u32,
    pub interface: [u8; 16],
    pub sub_index: u16,
    /// The broker record the core gave it, once given.
    pub record: Option<u64>,
    /// Its driver let go with GPADLs still up, and the core was told.
    pub noted: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Gpadl {
    id: u32,
    relid: u32,
}

/// What a host message changed for the broker: a record that no longer
/// names a live channel and must leave the table, or a record whose
/// channel the host holds no GPADL of any more, so pages kept for it
/// may go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retire {
    None,
    Record(u64),
    /// The host unloaded the bus: every record goes.
    All,
    /// The host tore down the channel's last GPADL.
    Settle(u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindError {
    /// No live offer with this relid.
    Unknown,
    /// Already given to a record.
    Bound,
    /// An interface no driver may be given.
    Interface,
}

pub(super) struct Channels {
    channels: [Option<Channel>; MAX_CHANNELS],
    gpadls: [Option<Gpadl>; MAX_GPADLS],
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

fn u16_at(p: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(p.get(at..at + 2)?.try_into().ok()?))
}

impl Channels {
    pub(super) const fn new() -> Self {
        Self { channels: [None; MAX_CHANNELS], gpadls: [None; MAX_GPADLS] }
    }

    pub(super) fn get(&self, relid: u32) -> Option<&Channel> {
        self.channels.iter().flatten().find(|c| c.relid == relid)
    }

    pub(super) fn by_record(&self, record: u64) -> Option<&Channel> {
        self.channels.iter().flatten().find(|c| c.record == Some(record))
    }

    /// The relid a GPADL id was created for.
    pub(super) fn gpadl_relid(&self, id: u32) -> Option<u32> {
        self.gpadls.iter().flatten().find(|g| g.id == id).map(|g| g.relid)
    }

    fn drop_gpadls_of(&mut self, relid: u32) {
        for g in self.gpadls.iter_mut() {
            if g.is_some_and(|g| g.relid == relid) {
                *g = None;
            }
        }
    }

    fn forget(&mut self, relid: u32) -> Retire {
        self.drop_gpadls_of(relid);
        let Some(at) = self.channels.iter().position(|c| c.is_some_and(|c| c.relid == relid))
        else {
            return Retire::None;
        };
        let record = self.channels[at].and_then(|c| c.record);
        self.channels[at] = None;
        record.map_or(Retire::None, Retire::Record)
    }

    /// A channel message from the host (the SynIC payload, its header
    /// already off). An offer is kept, or replaces one with its relid; a
    /// withdrawal forgets the channel and its GPADLs; a torn-down GPADL is
    /// forgotten; an unload forgets everything.
    pub(super) fn from_host(&mut self, p: &[u8]) -> Retire {
        let Some(kind) = u32_at(p, 0) else { return Retire::None };
        match kind {
            OFFERCHANNEL if p.len() >= OFFER_BYTES => {
                let (Some(relid), Some(connection), Some(sub)) =
                    (u32_at(p, 184), u32_at(p, 192), u16_at(p, 180))
                else {
                    return Retire::None;
                };
                if relid > MAX_RELID {
                    return Retire::None;
                }
                let mut interface = [0u8; 16];
                interface.copy_from_slice(&p[8..24]);
                let retire = self.forget(relid);
                let fresh = Channel {
                    relid,
                    connection,
                    interface,
                    sub_index: sub,
                    record: None,
                    noted: false,
                };
                if let Some(free) = self.channels.iter_mut().find(|c| c.is_none()) {
                    *free = Some(fresh);
                }
                retire
            }
            RESCIND_CHANNELOFFER => u32_at(p, 8).map_or(Retire::None, |r| self.forget(r)),
            GPADL_TORNDOWN => {
                let Some(id) = u32_at(p, 8) else { return Retire::None };
                let Some(relid) = self.gpadl_relid(id) else { return Retire::None };
                for g in self.gpadls.iter_mut() {
                    if g.is_some_and(|g| g.id == id) {
                        *g = None;
                    }
                }
                if self.has_gpadls(relid) {
                    return Retire::None;
                }
                match self.channels.iter_mut().flatten().find(|c| c.relid == relid) {
                    Some(c) => {
                        c.noted = false;
                        c.record.map_or(Retire::None, Retire::Settle)
                    }
                    None => Retire::None,
                }
            }
            UNLOAD_RESPONSE => {
                *self = Self::new();
                Retire::All
            }
            _ => Retire::None,
        }
    }

    /// A channel message the core posted and the host took. A GPADL
    /// header ties its id to the channel it names. A teardown unties
    /// nothing: the GPADL is the host's until it says GPADL_TORNDOWN.
    pub(super) fn from_guest(&mut self, p: &[u8]) {
        let (Some(kind), Some(relid), Some(id)) = (u32_at(p, 0), u32_at(p, 8), u32_at(p, 12))
        else {
            return;
        };
        if kind == GPADL_HEADER && self.gpadl_relid(id).is_none() {
            if let Some(free) = self.gpadls.iter_mut().find(|g| g.is_none()) {
                *free = Some(Gpadl { id, relid });
            }
        }
    }

    /// Whether the host may hold a GPADL of this channel: one was made and
    /// not yet reported torn down.
    pub(super) fn has_gpadls(&self, relid: u32) -> bool {
        self.gpadls.iter().flatten().any(|g| g.relid == relid)
    }

    /// A bound channel's driver let go: whether the host may still reach
    /// its pages, and whether this is the first word of it, for the core
    /// to be told once.
    pub(super) fn released(&mut self, record: u64) -> (bool, Option<u32>) {
        let Some(c) = self.channels.iter().flatten().find(|c| c.record == Some(record)).copied()
        else {
            return (false, None);
        };
        if !self.has_gpadls(c.relid) {
            return (false, None);
        }
        let first = self
            .channels
            .iter_mut()
            .flatten()
            .find(|x| x.relid == c.relid)
            .is_some_and(|x| !core::mem::replace(&mut x.noted, true));
        (true, first.then_some(c.relid))
    }

    /// The note to the core did not go: the next drain says it again.
    pub(super) fn unnote(&mut self, relid: u32) {
        if let Some(c) = self.channels.iter_mut().flatten().find(|c| c.relid == relid) {
            c.noted = false;
        }
    }

    /// Whether a header may make GPADL `id`: no live GPADL has that id (a
    /// reuse would untie the old one while the host still holds it), and
    /// there is room, so a header the host accepts is always remembered.
    pub(super) fn gpadl_free(&self, id: u32) -> bool {
        self.gpadl_relid(id).is_none() && self.gpadls.iter().any(Option::is_none)
    }

    /// Give a live, offered channel to a broker record.
    pub(super) fn bind(&mut self, relid: u32, record: u64) -> Result<Channel, BindError> {
        let c = self
            .channels
            .iter_mut()
            .flatten()
            .find(|c| c.relid == relid)
            .ok_or(BindError::Unknown)?;
        if c.record.is_some() {
            return Err(BindError::Bound);
        }
        if Interface::of(&c.interface).is_none() {
            return Err(BindError::Interface);
        }
        c.record = Some(record);
        Ok(*c)
    }

    /// The record of the channel a GPADL or open message is for, which must
    /// be bound: a header names its channel; a body names only its GPADL
    /// id, tied to a channel by the header before it; an OPENCHANNEL names
    /// both, and the GPADL must be that channel's. So only a channel given
    /// to a driver is ever built or opened, and never a host service.
    pub(super) fn target(&self, p: &[u8]) -> Option<u64> {
        let kind = u32_at(p, 0)?;
        let relid = match kind {
            GPADL_HEADER => u32_at(p, 8)?,
            GPADL_BODY => self.gpadl_relid(u32_at(p, 12)?)?,
            OPENCHANNEL => {
                let relid = u32_at(p, 8)?;
                if self.gpadl_relid(u32_at(p, 16)?) != Some(relid) {
                    return None;
                }
                relid
            }
            _ => return None,
        };
        self.get(relid)?.record
    }

    /// Every bound relid's bit, word `w` of the event flags.
    pub(super) fn bound_mask(&self, w: usize) -> u64 {
        self.channels
            .iter()
            .flatten()
            .filter(|c| c.record.is_some() && c.relid as usize / 64 == w)
            .fold(0, |m, c| m | 1u64 << (c.relid % 64))
    }
}

impl Default for Channels {
    fn default() -> Self {
        Self::new()
    }
}
