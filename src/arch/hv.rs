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

//! The hypervisor interfaces the kernel serves on every target, so a caller
//! keeps one path: the VMware backdoor's call, Hyper-V's SynIC for VMBus, and
//! the boot line naming the host.

#[path = "hv_backdoor.rs"]
mod hv_backdoor;
#[path = "hv_vmbus.rs"]
mod hv_vmbus;
#[path = "report_hypervisor.rs"]
mod report_hypervisor;

pub use hv_backdoor::hv_backdoor_present;
pub(crate) use hv_backdoor::{hv_backdoor, HvBackdoorError};
pub use hv_vmbus::hv_vmbus_ready;
pub(crate) use hv_vmbus::{
    hv_vmbus_bind, hv_vmbus_channel_device, hv_vmbus_channel_event, hv_vmbus_channel_of,
    hv_vmbus_events, hv_vmbus_post, hv_vmbus_post_channel, hv_vmbus_recv, hv_vmbus_recv_channel,
    hv_vmbus_released, hv_vmbus_retires, hv_vmbus_signal, HvRetire, HvVmbusError, HV_EVENT_WORDS,
    HV_MESSAGE_BYTES,
};
pub(crate) use report_hypervisor::report_hypervisor;
