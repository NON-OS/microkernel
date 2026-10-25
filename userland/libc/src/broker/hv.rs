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

//! The hypervisor calls: the VMware backdoor's one call, for the absolute
//! pointer, and the VMBus core's SynIC calls on Hyper-V.

#[path = "backdoor.rs"]
mod backdoor;
#[path = "vmbus.rs"]
mod vmbus;

pub use backdoor::mk_hv_backdoor;
pub use vmbus::{
    mk_hv_events, mk_hv_post, mk_hv_recv, mk_hv_signal, MK_HV_EVENT_WORDS, MK_HV_MESSAGE_BYTES,
    MK_VMBUS_DEVICE, MK_VMBUS_VENDOR,
};
