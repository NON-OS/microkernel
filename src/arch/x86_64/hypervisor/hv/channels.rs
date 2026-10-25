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
 * The VMBus channels as the kernel knows them, learnt from the host's own
 * offers as they pass, never from the core. A channel's connection id and
 * interface come from its offer.
 *
 * The core gives a channel to a driver by naming a relid the host offered
 * and has not withdrawn (`bind`). From then on the channel is its driver's:
 * - the driver posts that channel's messages, and only that channel's
 *   (`target`): GPADLs over pages held on its record, the open, the close,
 *   the teardowns;
 * - the host's answers for it come to the driver's own queue
 *   (`take_host_message`'s route), not to the core;
 * - the core can no longer reach the channel at all.
 *
 * A GPADL is tied to its channel from its header until the host says it is
 * gone (GPADL_TORNDOWN, or a failed GPADL_CREATED). While one is tied, the
 * host may write the channel's pages. So when the driver lets go, its pages
 * are kept and the kernel itself closes the channel and tears its GPADLs
 * down (`release`). Pure: kernel proofs run it.
 */

/// Channels kept at once; the core keeps as many.
pub(super) const MAX_CHANNELS: usize = 64;
/// GPADLs being built or held at once.
pub(super) const MAX_GPADLS: usize = 64;
/// The relids an event flag page can signal: 2048 bits.
pub(super) const MAX_RELID: u32 = 2047;
/// Host answers a channel's queue holds, and the bytes kept of each: the
/// longest, OPENCHANNEL_RESULT and GPADL_CREATED, are 20.
pub(super) const QUEUE: usize = 8;
pub(super) const REPLY_BYTES: usize = 32;

const OFFERCHANNEL: u32 = 1;
const RESCIND_CHANNELOFFER: u32 = 2;
const REQUESTOFFERS: u32 = 3;
const OPENCHANNEL: u32 = 5;
const OPENCHANNEL_RESULT: u32 = 6;
const CLOSECHANNEL: u32 = 7;
const GPADL_HEADER: u32 = 8;
const GPADL_BODY: u32 = 9;
const GPADL_CREATED: u32 = 10;
const GPADL_TEARDOWN: u32 = 11;
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
    /// An OPENCHANNEL went and no CLOSECHANNEL after it.
    pub open: bool,
    /// Its driver let go with GPADLs up; the kernel's close and teardowns
    /// are owed, or out and awaited.
    pub releasing: bool,
    /// The kernel's close and teardowns have gone to the host.
    pub cleaned: bool,
    queue: [(u8, [u8; REPLY_BYTES]); QUEUE],
    head: u8,
    len: u8,
}

impl Channel {
    fn new(relid: u32, connection: u32, interface: [u8; 16], sub_index: u16) -> Self {
        Self {
            relid,
            connection,
            interface,
            sub_index,
            record: None,
            open: false,
            releasing: false,
            cleaned: false,
            queue: [(0, [0; REPLY_BYTES]); QUEUE],
            head: 0,
            len: 0,
        }
    }

    /// False when full: the answer is dropped, and the driver's wait for it
    /// times out.
    fn push(&mut self, p: &[u8]) -> bool {
        if self.len as usize == QUEUE || p.len() > REPLY_BYTES {
            return false;
        }
        let at = (self.head as usize + self.len as usize) % QUEUE;
        let mut b = [0u8; REPLY_BYTES];
        b[..p.len()].copy_from_slice(p);
        self.queue[at] = (p.len() as u8, b);
        self.len += 1;
        true
    }

    fn pop(&mut self) -> Option<(usize, [u8; REPLY_BYTES])> {
        if self.len == 0 {
            return None;
        }
        let (n, b) = self.queue[self.head as usize];
        self.head = ((self.head as usize + 1) % QUEUE) as u8;
        self.len -= 1;
        Some((n as usize, b))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Gpadl {
    id: u32,
    relid: u32,
}

/// What a host message changed for the broker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Retire {
    None,
    /// A record that no longer names a live channel leaves the table.
    Record(u64),
    /// The host unloaded the bus: every record goes.
    All,
    /// The host holds no GPADL of the record's channel any more, so pages
    /// kept for it may go.
    Settle(u64),
}

/// Where a host message goes: the core's ring, or a channel's own queue
/// (already put there).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Route {
    Core,
    Channel(u32),
    /// A channel's answer its full queue could not take.
    Dropped(u32),
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

/// Why a driver's post for its channel is refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetError {
    /// Not one of a channel's messages, or too short to read.
    Kind,
    /// For another channel, or a GPADL not this channel's.
    Channel,
    /// A GPADL id already live.
    Reused,
    /// No room to remember another GPADL.
    Full,
    /// The record names no channel now.
    Gone,
    /// The bus is not up: no connection to post on.
    NoBus,
    /// The last driver's GPADLs are still being torn down.
    Releasing,
}

pub(super) struct Channels {
    channels: [Option<Channel>; MAX_CHANNELS],
    gpadls: [Option<Gpadl>; MAX_GPADLS],
    /// The connection the host takes channel messages on, as the core's
    /// request for offers went.
    bus: Option<u32>,
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

fn u16_at(p: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(p.get(at..at + 2)?.try_into().ok()?))
}

/// One of the kernel's own messages for a channel, and its length.
pub(super) type Owed = ([u8; 16], usize);

impl Channels {
    pub(super) const fn new() -> Self {
        Self { channels: [None; MAX_CHANNELS], gpadls: [None; MAX_GPADLS], bus: None }
    }

    pub(super) fn bus(&self) -> Option<u32> {
        self.bus
    }

    pub(super) fn get(&self, relid: u32) -> Option<&Channel> {
        self.channels.iter().flatten().find(|c| c.relid == relid)
    }

    fn get_mut(&mut self, relid: u32) -> Option<&mut Channel> {
        self.channels.iter_mut().flatten().find(|c| c.relid == relid)
    }

    pub(super) fn by_record(&self, record: u64) -> Option<&Channel> {
        self.channels.iter().flatten().find(|c| c.record == Some(record))
    }

    /// The relid a GPADL id was made for.
    pub(super) fn gpadl_relid(&self, id: u32) -> Option<u32> {
        self.gpadls.iter().flatten().find(|g| g.id == id).map(|g| g.relid)
    }

    /// Whether the host may hold a GPADL of this channel: one was made and
    /// not yet reported gone.
    pub(super) fn has_gpadls(&self, relid: u32) -> bool {
        self.gpadls.iter().flatten().any(|g| g.relid == relid)
    }

    fn bound(&self, relid: u32) -> bool {
        self.get(relid).is_some_and(|c| c.record.is_some())
    }

    fn untie(&mut self, id: u32) -> Retire {
        let Some(relid) = self.gpadl_relid(id) else { return Retire::None };
        for g in self.gpadls.iter_mut() {
            if g.is_some_and(|g| g.id == id) {
                *g = None;
            }
        }
        if self.has_gpadls(relid) {
            return Retire::None;
        }
        match self.get_mut(relid) {
            Some(c) => {
                if c.releasing {
                    c.releasing = false;
                    c.cleaned = false;
                }
                c.record.map_or(Retire::None, Retire::Settle)
            }
            None => Retire::None,
        }
    }

    fn forget(&mut self, relid: u32) -> Retire {
        for g in self.gpadls.iter_mut() {
            if g.is_some_and(|g| g.relid == relid) {
                *g = None;
            }
        }
        let Some(at) = self.channels.iter().position(|c| c.is_some_and(|c| c.relid == relid))
        else {
            return Retire::None;
        };
        let record = self.channels[at].and_then(|c| c.record);
        self.channels[at] = None;
        record.map_or(Retire::None, Retire::Record)
    }

    /// A channel message from the host (the SynIC payload). An offer is
    /// kept, or replaces one with its relid; a withdrawal forgets the
    /// channel and its GPADLs; an unload forgets everything. A bound
    /// channel's answers (its open's result, its GPADLs made or gone) go
    /// to its queue; a GPADL the host reports gone, or failed to make, is
    /// untied.
    pub(super) fn take_host_message(&mut self, p: &[u8]) -> (Route, Retire) {
        let Some(kind) = u32_at(p, 0) else { return (Route::Core, Retire::None) };
        let to = |relid: Option<u32>, me: &Self| relid.filter(|r| me.bound(*r));
        match kind {
            OFFERCHANNEL if p.len() >= OFFER_BYTES => {
                let (Some(relid), Some(connection), Some(sub)) =
                    (u32_at(p, 184), u32_at(p, 192), u16_at(p, 180))
                else {
                    return (Route::Core, Retire::None);
                };
                if relid > MAX_RELID {
                    return (Route::Core, Retire::None);
                }
                let mut interface = [0u8; 16];
                interface.copy_from_slice(&p[8..24]);
                let retire = self.forget(relid);
                if let Some(free) = self.channels.iter_mut().find(|c| c.is_none()) {
                    *free = Some(Channel::new(relid, connection, interface, sub));
                }
                (Route::Core, retire)
            }
            RESCIND_CHANNELOFFER => {
                (Route::Core, u32_at(p, 8).map_or(Retire::None, |r| self.forget(r)))
            }
            OPENCHANNEL_RESULT => match to(u32_at(p, 8), self) {
                Some(relid) => (self.queue(relid, p), Retire::None),
                None => (Route::Core, Retire::None),
            },
            GPADL_CREATED => {
                let target = to(u32_at(p, 8), self);
                let failed = u32_at(p, 16).is_some_and(|s| s != 0);
                let id = u32_at(p, 12);
                let retire = match (failed, id) {
                    (true, Some(id)) if self.gpadl_relid(id) == u32_at(p, 8) => self.untie(id),
                    _ => Retire::None,
                };
                match target {
                    Some(relid) => (self.queue(relid, p), retire),
                    None => (Route::Core, retire),
                }
            }
            GPADL_TORNDOWN => {
                let Some(id) = u32_at(p, 8) else { return (Route::Core, Retire::None) };
                let target = to(self.gpadl_relid(id), self);
                let quiet = target.and_then(|r| self.get(r)).is_some_and(|c| c.releasing);
                let retire = self.untie(id);
                match target {
                    // The kernel's own teardown: its driver is gone.
                    Some(_) if quiet => (Route::Core, retire),
                    Some(relid) => (self.queue(relid, p), retire),
                    None => (Route::Core, retire),
                }
            }
            UNLOAD_RESPONSE => {
                *self = Self::new();
                (Route::Core, Retire::All)
            }
            _ => (Route::Core, Retire::None),
        }
    }

    fn queue(&mut self, relid: u32, p: &[u8]) -> Route {
        if self.get_mut(relid).is_some_and(|c| c.push(p)) {
            Route::Channel(relid)
        } else {
            Route::Dropped(relid)
        }
    }

    /// The oldest answer waiting in a record's channel's queue.
    pub(super) fn take(&mut self, record: u64) -> Option<(usize, [u8; REPLY_BYTES])> {
        self.channels.iter_mut().flatten().find(|c| c.record == Some(record))?.pop()
    }

    /// A message the core posted and the host took: its request for offers
    /// names the bus's connection.
    pub(super) fn take_core_message(&mut self, connection: u32, p: &[u8]) {
        if u32_at(p, 0) == Some(REQUESTOFFERS) {
            self.bus = Some(connection);
        }
    }

    /// The channel a driver's message is for, which must be its record's:
    /// an open (with one of the channel's own GPADLs), a close, a GPADL's
    /// header, body or teardown. A header must make a new id, with room
    /// to remember it.
    pub(super) fn target(&self, record: u64, p: &[u8]) -> Result<u32, TargetError> {
        let c = self.by_record(record).ok_or(TargetError::Gone)?;
        if c.releasing {
            return Err(TargetError::Releasing);
        }
        let mine = c.relid;
        if self.bus.is_none() {
            return Err(TargetError::NoBus);
        }
        let kind = u32_at(p, 0).ok_or(TargetError::Kind)?;
        let named = || u32_at(p, 8).ok_or(TargetError::Kind);
        let ours = |r: u32| if r == mine { Ok(r) } else { Err(TargetError::Channel) };
        let tied = |id: Option<u32>| match id.and_then(|i| self.gpadl_relid(i)) {
            Some(r) if r == mine => Ok(mine),
            _ => Err(TargetError::Channel),
        };
        match kind {
            CLOSECHANNEL => ours(named()?),
            OPENCHANNEL => {
                ours(named()?)?;
                tied(u32_at(p, 16))
            }
            GPADL_HEADER => {
                ours(named()?)?;
                let id = u32_at(p, 12).ok_or(TargetError::Kind)?;
                if self.gpadl_relid(id).is_some() {
                    return Err(TargetError::Reused);
                }
                if !self.gpadls.iter().any(Option::is_none) {
                    return Err(TargetError::Full);
                }
                Ok(mine)
            }
            GPADL_BODY => tied(u32_at(p, 12)),
            GPADL_TEARDOWN => {
                ours(named()?)?;
                tied(u32_at(p, 12))
            }
            _ => Err(TargetError::Kind),
        }
    }

    /// A driver's message for its channel that the host took: a header ties
    /// its GPADL, an open opens and a close closes. A teardown unties
    /// nothing: the GPADL is the host's until it says GPADL_TORNDOWN.
    pub(super) fn take_driver_message(&mut self, p: &[u8]) {
        let (Some(kind), Some(relid)) = (u32_at(p, 0), u32_at(p, 8)) else { return };
        match kind {
            GPADL_HEADER => {
                let Some(id) = u32_at(p, 12) else { return };
                if self.gpadl_relid(id).is_none() {
                    if let Some(free) = self.gpadls.iter_mut().find(|g| g.is_none()) {
                        *free = Some(Gpadl { id, relid });
                    }
                }
            }
            OPENCHANNEL => {
                if let Some(c) = self.get_mut(relid) {
                    c.open = true;
                }
            }
            CLOSECHANNEL => {
                if let Some(c) = self.get_mut(relid) {
                    c.open = false;
                }
            }
            _ => {}
        }
    }

    /// Give a live, offered channel to a broker record.
    pub(super) fn bind(&mut self, relid: u32, record: u64) -> Result<Channel, BindError> {
        let c = self.get_mut(relid).ok_or(BindError::Unknown)?;
        if c.record.is_some() {
            return Err(BindError::Bound);
        }
        if Interface::of(&c.interface).is_none() {
            return Err(BindError::Interface);
        }
        c.record = Some(record);
        Ok(*c)
    }

    /// A bound channel's driver let go: whether the host may still reach
    /// its pages, so they are kept. If so, the channel is releasing until
    /// the host says its last GPADL is gone.
    pub(super) fn release(&mut self, record: u64) -> bool {
        let Some(relid) = self.by_record(record).map(|c| c.relid) else { return false };
        if !self.has_gpadls(relid) {
            return false;
        }
        if let Some(c) = self.get_mut(relid) {
            c.releasing = true;
            c.queue = [(0, [0; REPLY_BYTES]); QUEUE];
            c.head = 0;
            c.len = 0;
        }
        true
    }

    /// The kernel's own messages for a releasing channel whose driver is
    /// gone: its close if open, then a teardown of each GPADL still tied to
    /// it. Owed once; `cleaned` marks them sent, and a failed post clears
    /// it for the next look.
    pub(super) fn owed(&self, out: &mut [Owed; MAX_GPADLS + 1]) -> Option<(u32, usize)> {
        let c = self.channels.iter().flatten().find(|c| c.releasing && !c.cleaned)?;
        let mut n = 0;
        if c.open {
            let mut m = [0u8; 16];
            m[..4].copy_from_slice(&CLOSECHANNEL.to_le_bytes());
            m[8..12].copy_from_slice(&c.relid.to_le_bytes());
            out[n] = (m, 12);
            n += 1;
        }
        for g in self.gpadls.iter().flatten().filter(|g| g.relid == c.relid) {
            let mut m = [0u8; 16];
            m[..4].copy_from_slice(&GPADL_TEARDOWN.to_le_bytes());
            m[8..12].copy_from_slice(&c.relid.to_le_bytes());
            m[12..16].copy_from_slice(&g.id.to_le_bytes());
            out[n] = (m, 16);
            n += 1;
        }
        Some((c.relid, n))
    }

    /// The kernel's messages for a channel went (`ok`), or did not and are
    /// owed again.
    pub(super) fn cleaned(&mut self, relid: u32, ok: bool) {
        if let Some(c) = self.get_mut(relid) {
            c.cleaned = ok;
            if ok {
                c.open = false;
            }
        }
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
