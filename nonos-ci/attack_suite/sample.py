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

"""A synthetic serial log that passes, for the self-test and the unit
tests only. It is written here, line by line, as each printer formats its
lines; it is not a boot's log and no report may present it as one."""

from .rules import BOOTS

CAPSULE_LINES = """cap-escape refused: a socket through net.sockets without Network, MkIpcSend EPERM (1)
cap-escape refused: an MMIO window without Mmio, MkMmioMap EPERM (1)
cap-escape refused: claim a device without Driver, MkDeviceClaim EPERM (1)
cap-escape refused: read PCI config space without Driver, MkPciConfigRead EPERM (1)
cap-escape refused: take an I/O port range without Pio, MkPioGrant EPERM (1)
cap-escape refused: bind a device interrupt without Irq, MkIrqBind EPERM (1)
cap-escape refused: map a DMA buffer without Dma, MkDmaMap EPERM (1)
cap-escape refused: grant itself Network without Admin, MkCapGrant EPERM (1)
cap-escape refused: register net.dns without RegisterService, MkServiceRegister EPERM (1)
cap-escape refused: net.sockets' inbox by its pid without Network, MkIpcSendToPid EPERM (1)
cap-escape refused: a network card's driver by name, held to the stack, MkIpcSend EPERM (1)
cap-escape refused: a network card's driver by its pid, held to the stack, MkIpcSendToPid EPERM (1)
cap-escape refused: a device's driver by name, held to its service, MkIpcSend EPERM (1)
cap-escape refused: a device's driver by its pid, held to its service, MkIpcSendToPid EPERM (1)
foreign-memory refused: a kernel address to MkDebug EFAULT (14)
foreign-memory refused: another address space's address to MkDebug EFAULT (14)
foreign-memory refused: a kernel address as an IPC message, MkIpcSend EFAULT (14)
foreign-memory refused: a fixed mapping at a kernel address, MkMmap EPERM (1)
service-squat refused: MkServiceRegister net.sockets EPERM (1)
service-squat refused: MkServiceRegister app.file_manager EPERM (1)
held-escape refused: a held driver by its reply port, MkIpcSend EPERM (1)
held-escape refused: a net service by its reply port without Network, MkIpcSend EPERM (1)
held-escape refused: xhci's bulk-IN by its reply port, MkIpcCall EPERM (1)"""

FAULTED = """[ATTACK-FAULT] pid 41 writes address 0x0
[TRAP PF] cpl=3 rip=0x0000000000402832 rsp=0x00007fffffffe000 cs=0x0000000000000033 ss=0x000000000000002b rflags=0x0000000000010246 cr3=0x0000000001234000 asid=0x0000000000000009 pid=0x0000000000000029 err=0x0000000000000006 cr2=0x0000000000000000
[TRAP WALK] l4=0
[NET] dhcp lease renewed
"""


def passing():
    """One log per boot, in rules.BOOTS order: attest, capsule, linux."""
    capsule = ["[ATTACK-START] capsule pid 41\n"]
    for line in CAPSULE_LINES.splitlines():
        capsule.append("[KLOG WARN] [CAP-DENY] pid=41 syscall=MkIpcSend(1)\r\n")
        capsule.append(f"[ATTACK] {line}\n")
    attest = [f"[ATTACK] {a} refused: the proof does not verify code 3\n" for a in BOOTS["attest"]]
    linux = [f"[ATTACK] {a} refused: fs a path past the root, errno -2\n" for a in BOOTS["linux"]]
    return ["".join(attest), "".join(capsule) + FAULTED, "".join(linux)]
