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

//! The SynIC's calls for the VMBus core, for the syscall layer. Elsewhere
//! than x86_64 there is no SynIC, and every call answers that.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub(crate) enum HvVmbusError {
    /// No SynIC: another hypervisor, or another architecture.
    NoSynic,
    /// The message is not one the core may post (see `vmbus_out`).
    Refused,
    /// A page the message names is not the caller's.
    NotHeld,
    /// The host has no room for it now.
    Busy,
    /// Any other hypercall status.
    Failed,
}

pub(crate) const HV_MESSAGE_BYTES: usize = 256;
pub(crate) const HV_EVENT_WORDS: usize = 32;

#[cfg(target_arch = "x86_64")]
mod imp {
    use super::HvVmbusError;
    use crate::arch::x86_64::hypervisor::hv;

    const _: () = assert!(
        super::HV_MESSAGE_BYTES == hv::MESSAGE_BYTES && super::HV_EVENT_WORDS == hv::EVENT_WORDS
    );

    fn result(status: Option<u16>) -> Result<(), HvVmbusError> {
        match status {
            None => Err(HvVmbusError::NoSynic),
            Some(hv::STATUS_SUCCESS) => Ok(()),
            Some(hv::STATUS_INSUFFICIENT_BUFFERS) => Err(HvVmbusError::Busy),
            Some(_) => Err(HvVmbusError::Failed),
        }
    }

    pub(crate) fn post(
        connection: u32,
        ty: u32,
        payload: &[u8],
        held: impl Fn(u64) -> bool,
    ) -> Result<(), HvVmbusError> {
        if !hv::ready() {
            return Err(HvVmbusError::NoSynic);
        }
        hv::check(connection, ty, payload, held).map_err(|e| match e {
            hv::Refusal::Page => HvVmbusError::NotHeld,
            _ => HvVmbusError::Refused,
        })?;
        result(hv::post(connection, ty, payload))
    }

    pub(crate) fn recv(out: &mut [u8; super::HV_MESSAGE_BYTES]) -> Option<usize> {
        hv::recv(out)
    }

    pub(crate) fn events(out: &mut [u64]) -> usize {
        hv::events(out)
    }

    pub(crate) fn signal(connection: u32) -> Result<(), HvVmbusError> {
        result(hv::signal(connection))
    }

    pub fn ready() -> bool {
        hv::ready()
    }
}

#[cfg(not(target_arch = "x86_64"))]
mod imp {
    use super::HvVmbusError;

    pub(crate) fn post(
        _connection: u32,
        _ty: u32,
        _payload: &[u8],
        _held: impl Fn(u64) -> bool,
    ) -> Result<(), HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub(crate) fn recv(_out: &mut [u8; super::HV_MESSAGE_BYTES]) -> Option<usize> {
        None
    }

    pub(crate) fn events(_out: &mut [u64]) -> usize {
        0
    }

    pub(crate) fn signal(_connection: u32) -> Result<(), HvVmbusError> {
        Err(HvVmbusError::NoSynic)
    }

    pub fn ready() -> bool {
        false
    }
}

/// Whether the SynIC is up, so the VMBus core is worth starting.
pub use imp::ready as hv_vmbus_ready;
pub(crate) use imp::{
    events as hv_vmbus_events, post as hv_vmbus_post, recv as hv_vmbus_recv,
    signal as hv_vmbus_signal,
};
