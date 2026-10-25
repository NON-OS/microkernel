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
 * The Hyper-V SynIC's pure parts, included by path: what the partition
 * must be offered, the MSR values, the hypercall words, a message slot,
 * the message ring, and what the VMBus core may post to the host.
 */

#[path = "../../../../src/arch/x86_64/hypervisor/hv/channels.rs"]
mod channels;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/features.rs"]
mod features;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/hypercall_abi.rs"]
mod hypercall_abi;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/message.rs"]
mod message;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/regs.rs"]
mod regs;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/ring.rs"]
mod ring;
mod tests;
mod tests_channels;
mod tests_vmbus;
#[path = "../../../../src/arch/x86_64/hypervisor/hv/vmbus_out.rs"]
mod vmbus_out;
