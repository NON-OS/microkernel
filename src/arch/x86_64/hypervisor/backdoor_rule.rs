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
 * The VMware backdoor: `in eax, dx` on its port with the magic in EAX
 * and the command in ECX, four registers back. A guest driver reaches it
 * for the absolute pointer alone. Any other command (the RPC channel, the
 * clipboard, the host's time and its guest info) carries data between the
 * guest and the host, which no capsule sends unless the user opts in, so
 * it is refused here rather than filtered in a driver.
 */

use super::vendor::Hypervisor;

pub(super) const BACKDOOR_PORT: u16 = 0x5658;
pub(super) const BACKDOOR_MAGIC: u32 = 0x564D_5868;

pub(super) const CMD_GETVERSION: u32 = 10;
pub(super) const CMD_ABSPOINTER_DATA: u32 = 39;
pub(super) const CMD_ABSPOINTER_STATUS: u32 = 40;
pub(super) const CMD_ABSPOINTER_COMMAND: u32 = 41;

pub(super) const ABSPOINTER_ENABLE: u32 = 0x4541_4552;
pub(super) const ABSPOINTER_DISABLE: u32 = 0x0000_00F5;
pub(super) const ABSPOINTER_RELATIVE: u32 = 0x4C45_5252;
pub(super) const ABSPOINTER_ABSOLUTE: u32 = 0x5342_4152;

/// Words one ABSPOINTER_DATA call may take from the host's queue.
pub(super) const ABSPOINTER_DATA_MAX: u32 = 4;

/*
 * VMware answers on the port, and QEMU's vmport device does under KVM and
 * TCG alike. Elsewhere the port is unassigned at best and, on a physical
 * machine, may belong to a device.
 */
pub(super) const fn host_answers(h: Hypervisor) -> bool {
    matches!(h, Hypervisor::Vmware | Hypervisor::Kvm | Hypervisor::QemuTcg)
}

pub(super) const fn call_allowed(cmd: u32, arg: u32) -> bool {
    match cmd {
        CMD_GETVERSION | CMD_ABSPOINTER_STATUS => true,
        CMD_ABSPOINTER_DATA => arg >= 1 && arg <= ABSPOINTER_DATA_MAX,
        CMD_ABSPOINTER_COMMAND => matches!(
            arg,
            ABSPOINTER_ENABLE | ABSPOINTER_DISABLE | ABSPOINTER_RELATIVE | ABSPOINTER_ABSOLUTE
        ),
        _ => false,
    }
}
