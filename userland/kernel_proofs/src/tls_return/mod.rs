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
 * A parked guest returns to user mode on the thread pointer its supervisor
 * last set, whether or not it switched out while it waited.
 *
 * MkPeerTls (src/process/foreign/peer_tls.rs) writes the base into the
 * control block only; the scheduler's switch writes the control block's
 * base to MSR_FS_BASE. On several CPUs a guest can be answered while it is
 * still in trap_wait, and then no switch runs between its arch_prctl and
 * its return. A shell that exec'd john on the last command of `sh -c` did
 * exactly that: john's arch_prctl(ARCH_SET_FS) was answered, john ran on
 * the FS 0 its exec left, and its first TLS read (`mov %fs:-4`) faulted at
 * 0xfffffffffffffffc. The model below is the CPU's FS register, a control
 * block and the two ways back to user mode; the source checks pin the
 * kernel to the rule the model proves.
 */

mod tests;
