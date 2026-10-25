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

//! The SynIC's calls for VMBus, for the syscall layer: the core's four, and
//! the channels it gives drivers. Elsewhere than x86_64 there is no SynIC,
//! and every call answers that.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub(crate) enum HvVmbusError {
    /// No SynIC: another hypervisor, or another architecture.
    NoSynic,
    /// The message is not one the core may post (see `vmbus_out`).
    Refused,
    /// A page the message names is not held as it must be.
    NotHeld,
    /// A GPADL or open for no channel given to a driver.
    NoChannel,
    /// The host has no room for it now.
    Busy,
    /// Any other hypercall status.
    Failed,
    /// No live offer with this relid.
    Unknown,
    /// The channel is already given out.
    Bound,
    /// An interface no driver may be given (a host service).
    Interface,
}

/// Who must hold a page a posted message names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub(crate) enum HvHolder {
    /// The caller, on the VMBus root.
    Root,
    /// Anyone, on this channel record.
    Record(u64),
}

/// A channel record a host message retired.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub(crate) enum HvRetire {
    None,
    Record(u64),
    All,
    /// The host tore down the channel's last GPADL: pages kept for it may go.
    Settle(u64),
}

/// A channel given to a record: its relid and the connection the host
/// named for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub(crate) struct HvChannel {
    pub relid: u32,
    pub connection: u32,
}

pub(crate) const HV_MESSAGE_BYTES: usize = 256;
pub(crate) const HV_EVENT_WORDS: usize = 32;

#[cfg(target_arch = "x86_64")]
mod imp {
    use super::{HvChannel, HvHolder, HvRetire, HvVmbusError};
    use crate::arch::x86_64::hypervisor::hv;

    const _: () = assert!(
        super::HV_MESSAGE_BYTES == hv::MESSAGE_BYTES && super::HV_EVENT_WORDS == hv::EVENT_WORDS
    );

    fn status(s: u16) -> HvVmbusError {
        if s == hv::STATUS_INSUFFICIENT_BUFFERS {
            HvVmbusError::Busy
        } else {
            HvVmbusError::Failed
        }
    }

    fn bind_error(e: hv::BindError) -> HvVmbusError {
        match e {
            hv::BindError::Unknown => HvVmbusError::Unknown,
            hv::BindError::Bound => HvVmbusError::Bound,
            hv::BindError::Interface => HvVmbusError::Interface,
        }
    }

    pub(crate) fn post(
        connection: u32,
        ty: u32,
        payload: &[u8],
        held: impl Fn(HvHolder, u64) -> bool,
    ) -> Result<(), HvVmbusError> {
        if !hv::ready() {
            return Err(HvVmbusError::NoSynic);
        }
        let held = |h: hv::Holder, gpa: u64| match h {
            hv::Holder::Root => held(HvHolder::Root, gpa),
            hv::Holder::Record(r) => held(HvHolder::Record(r), gpa),
        };
        hv::post_checked(connection, ty, payload, held).map_err(|e| match e {
            hv::PostError::Refused(hv::Refusal::Page) => HvVmbusError::NotHeld,
            hv::PostError::Refused(hv::Refusal::Channel) => HvVmbusError::NoChannel,
            hv::PostError::Refused(_) => HvVmbusError::Refused,
            hv::PostError::NoSynic => HvVmbusError::NoSynic,
            hv::PostError::Status(s) => status(s),
        })
    }

    pub(crate) fn recv(out: &mut [u8; super::HV_MESSAGE_BYTES]) -> Option<(usize, HvRetire)> {
        let (n, retire) = hv::recv(out)?;
        let retire = match retire {
            hv::Retire::None => HvRetire::None,
            hv::Retire::Record(r) => HvRetire::Record(r),
            hv::Retire::All => HvRetire::All,
            hv::Retire::Settle(r) => HvRetire::Settle(r),
        };
        Some((n, retire))
    }

    pub(crate) fn events(out: &mut [u64]) -> usize {
        hv::events(out)
    }

    pub(crate) fn signal(connection: u32) -> Result<(), HvVmbusError> {
        match hv::signal(connection) {
            None => Err(HvVmbusError::NoSynic),
            Some(hv::STATUS_SUCCESS) => Ok(()),
            Some(s) => Err(status(s)),
        }
    }

    /// The device id a live, unbound channel's record would carry.
    pub(crate) fn channel_device(relid: u32) -> Result<u16, HvVmbusError> {
        if !hv::ready() {
            return Err(HvVmbusError::NoSynic);
        }
        hv::channel_interface(relid).map(|i| i.device()).map_err(bind_error)
    }

    pub(crate) fn bind(relid: u32, record: u64) -> Result<HvChannel, HvVmbusError> {
        hv::bind(relid, record)
            .map(|c| HvChannel { relid: c.relid, connection: c.connection })
            .map_err(bind_error)
    }

    pub(crate) fn channel_of(record: u64) -> Option<HvChannel> {
        hv::channel_of(record).map(|c| HvChannel { relid: c.relid, connection: c.connection })
    }

    pub(crate) fn channel_event(relid: u32) -> bool {
        hv::channel_event(relid)
    }

    pub(crate) fn released(record: u64) -> bool {
        hv::released(record)
    }

    pub fn ready() -> bool {
        hv::ready()
    }
}

#[cfg(not(target_arch = "x86_64"))]
mod imp {
    use super::{HvChannel, HvHolder, HvRetire, HvVmbusError};

    pub(crate) fn post(
        _connection: u32,
        _ty: u32,
        _payload: &[u8],
        _held: impl Fn(HvHolder, u64) -> bool,
    ) -> Result<(), HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub(crate) fn recv(_out: &mut [u8; super::HV_MESSAGE_BYTES]) -> Option<(usize, HvRetire)> {
        None
    }

    pub(crate) fn events(_out: &mut [u64]) -> usize {
        0
    }

    pub(crate) fn signal(_connection: u32) -> Result<(), HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub(crate) fn channel_device(_relid: u32) -> Result<u16, HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub(crate) fn bind(_relid: u32, _record: u64) -> Result<HvChannel, HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub(crate) fn channel_of(_record: u64) -> Option<HvChannel> {
        None
    }

    pub(crate) fn channel_event(_relid: u32) -> bool {
        false
    }

    pub(crate) fn released(_record: u64) -> bool {
        false
    }

    pub fn ready() -> bool {
        false
    }
}

/// Whether the SynIC is up, so the VMBus core is worth starting.
pub use imp::ready as hv_vmbus_ready;
pub(crate) use imp::{
    bind as hv_vmbus_bind, channel_device as hv_vmbus_channel_device,
    channel_event as hv_vmbus_channel_event, channel_of as hv_vmbus_channel_of,
    events as hv_vmbus_events, post as hv_vmbus_post, recv as hv_vmbus_recv,
    released as hv_vmbus_released, signal as hv_vmbus_signal,
};
