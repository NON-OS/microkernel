# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""The isolation matrix rows each refusal has to prove, for rules.py. Data
only."""

# Each matrix row, the attack that carries it, and every refusal it needs: a
# pattern each must match in some refused line of that attack. Each names the
# errno the gate answers with, so a call refused for another reason (no such
# service, a renumbered syscall) proves no row.
ROWS = (
    ("cap-escape: Network", "cap-escape", (r"net\.sockets without Network, MkIpcSend EPERM",)),
    ("cap-escape: Mmio", "cap-escape", (r"without Mmio, MkMmioMap EPERM",)),
    # The Hardware bit gates no call (capabilities/types/defs.rs); the
    # hardware operations are gated by the finer bits, each tried here.
    ("cap-escape: Hardware", "cap-escape",
     (r"MkDeviceClaim EPERM", r"MkPciConfigRead EPERM", r"MkPioGrant EPERM", r"MkIrqBind EPERM",
      r"MkDmaMap EPERM")),
    ("cap-escape: Admin", "cap-escape", (r"without Admin, MkCapGrant EPERM",)),
    ("cap-escape: RegisterService", "cap-escape", (r"without RegisterService, MkServiceRegister EPERM",)),
    # Every endpoint a process serves is read from its one inbox, so a send by
    # pid passes the same gates as a send by name.
    ("cap-escape: an inbox by pid", "cap-escape",
     (r"net\.sockets' inbox by its pid without Network, MkIpcSendToPid EPERM",)),
    # A card's driver is held to the network stack (registry/held.rs).
    ("cap-escape: a network card", "cap-escape",
     (r"network card's driver by name, .* EPERM", r"network card's driver by its pid, .* EPERM")),
    # Keyboards, controllers, the GPU and the sound card, held the same way.
    ("cap-escape: a device driver", "cap-escape",
     (r"device's driver by name, .* EPERM", r"device's driver by its pid, .* EPERM")),
    # Every name a process owns is read from its one inbox, a reply port too.
    ("held-escape: a reply port", "held-escape",
     (r"held driver by its reply port, .* EPERM", r"net service by its reply port without Network, .* EPERM",
      r"xhci's bulk-IN by its reply port, .* EPERM")),
    ("deputy: the shell's open-with", "deputy-escape",
     (r"app\.install through the shell's OP_OPEN_WITH ENOENT",)),
    ("foreign-memory: kernel via Debug", "foreign-memory", (r"a kernel address to MkDebug EFAULT",)),
    ("foreign-memory: other aspace via Debug", "foreign-memory",
     (r"another address space's address to MkDebug EFAULT",)),
    ("foreign-memory: kernel as IPC", "foreign-memory", (r"kernel address as an IPC message, MkIpcSend EFAULT",)),
    ("foreign-memory: fixed map", "foreign-memory", (r"fixed mapping at a kernel address, MkMmap EPERM",)),
    ("service-squat", "service-squat",
     (r"MkServiceRegister net\.sockets EPERM", r"MkServiceRegister app\.file_manager EPERM")),
    ("fault containment", "fault-containment", (r"the kernel ended pid \d+ at its fault",)),
    ("personality escape: fs", "linux-fs-escape", (r".",)),
    ("personality escape: memory", "linux-foreign-memory", (r".",)),
    ("personality escape: raw syscall", "linux-raw-syscall", (r".",)),
)
