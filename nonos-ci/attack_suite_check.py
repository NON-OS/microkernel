#!/usr/bin/env python3
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

"""Pass only when every attack in the attack suite was attempted and
refused, with "[ATTACK] <name> refused: <how>", and none logged "ESCAPED".
An attack that never ran fails the run, and so does a row of the isolation
matrix with no refusal of its
own. The attacker's fault cannot be reported by the attacker, so its line is
judged here from the kernel's trap line and what the log did after it."""

import argparse
import re
import sys

ATTACKS = """trailer-flip trailer-extra-cap trailer-kernel-kind trailer-stale-epoch cap-escape
foreign-memory service-squat kernel-replay chainload linux-fs-escape linux-foreign-memory
linux-raw-syscall fault-containment""".split()
LINE = re.compile(r"\[ATTACK\] ([a-z0-9-]+) (refused|ESCAPED): ?([^\n]*)")

# Each matrix row, the attack that carries it, and every refusal it needs.
ROWS = [
    ("cap-escape: Network", "cap-escape", [r"net\.sockets without Network"]),
    ("cap-escape: Mmio", "cap-escape", [r"without Mmio, MkMmioMap"]),
    # The Hardware bit gates no call (capabilities/types/defs.rs); the
    # hardware operations are gated by the finer bits, each tried here.
    ("cap-escape: Hardware", "cap-escape", [r"MkDeviceClaim", r"MkPciConfigRead",
                                            r"MkPioGrant", r"MkIrqBind", r"MkDmaMap"]),
    ("cap-escape: Admin", "cap-escape", [r"without Admin, MkCapGrant"]),
    ("cap-escape: RegisterService", "cap-escape",
     [r"without RegisterService, MkServiceRegister"]),
    # Every endpoint a process serves is read from its one inbox, so a send by
    # pid passes the same gates as a send by name.
    ("cap-escape: an inbox by pid", "cap-escape",
     [r"net\.sockets' inbox by its pid without Network, MkIpcSendToPid"]),
    # A card's driver is held to the network stack (registry/held.rs).
    ("cap-escape: a network card", "cap-escape",
     [r"network card's driver by name", r"network card's driver by its pid"]),
    # Keyboards, controllers, the GPU and the sound card, held the same way.
    ("cap-escape: a device driver", "cap-escape",
     [r"device's driver by name", r"device's driver by its pid"]),
    ("foreign-memory: kernel via Debug", "foreign-memory", [r"a kernel address to MkDebug"]),
    ("foreign-memory: other aspace via Debug", "foreign-memory",
     [r"another address space's address to MkDebug"]),
    ("foreign-memory: kernel as IPC", "foreign-memory", [r"kernel address as an IPC message"]),
    ("foreign-memory: fixed map", "foreign-memory", [r"fixed mapping at a kernel address"]),
    ("service-squat", "service-squat", [r"MkServiceRegister net\.sockets",
                                        r"MkServiceRegister app\.file_manager"]),
    ("fault containment", "fault-containment", [r"the kernel ended pid \d+ at its fault"]),
    ("personality escape: fs", "linux-fs-escape", [r"."]),
    ("personality escape: memory", "linux-foreign-memory", [r"."]),
    ("personality escape: raw syscall", "linux-raw-syscall", [r"."]),
]

FAULT = re.compile(r"\[ATTACK-FAULT\] pid (\d+) writes address (0x[0-9a-f]+)")
TRAP = re.compile(r"\[TRAP PF\] cpl=3 .*?pid=(0x[0-9a-f]+).*?cr2=(0x[0-9a-f]+)")
# The kernel's own death: a Rust panic, or a fault with no recovery path
# (arch/x86_64/diag/fatal_notice.rs). A kernel-mode trap line alone is not
# one, the handler resolves demand faults on user copies.
PANIC = re.compile(r"KERNEL PANIC|\[PANIC [A-Z]+\] fatal kernel fault")


def fault_lines(text):
    """The attack lines the attacker's own fault earns, from the lines after
    its marker: the kernel's user-mode trap for that pid at that address,
    then the log going on with no kernel fault."""
    lines = text.splitlines()
    out = []
    for i, line in enumerate(lines):
        m = FAULT.search(line)
        if not m:
            continue
        pid, addr = int(m.group(1)), int(m.group(2), 16)
        rest = lines[i + 1:]
        trap = next((j for j, l in enumerate(rest) if (t := TRAP.search(l))
                     and int(t.group(1), 16) == pid and int(t.group(2), 16) == addr), None)
        if trap is None:
            out.append(f"[ATTACK] fault-containment ESCAPED: no user fault for pid {pid} at {hex(addr)}")
            continue
        after = [l for l in rest[trap + 1:] if l.strip() and not l.startswith("[TRAP")]
        if any(PANIC.search(l) for l in rest):
            out.append("[ATTACK] fault-containment ESCAPED: the kernel faulted after the attacker did")
        elif not after:
            out.append("[ATTACK] fault-containment ESCAPED: the log ended at the attacker's fault")
        else:
            out.append(f"[ATTACK] fault-containment refused: the kernel ended pid {pid} at its fault "
                       f"on {hex(addr)}, and {len(after)} lines followed")
    return "\n".join(out) + "\n" if out else ""


def unproven_rows(seen):
    bad = []
    for row, name, needs in ROWS:
        hows = [how for v, how in seen.get(name, []) if v == "refused"]
        for need in needs:
            if not any(re.search(need, how) for how in hows):
                bad.append(f"matrix row {row}: no refusal matching {need!r}")
    return bad


def judge(text):
    text += fault_lines(text)
    seen, bad = {}, []
    for name, verdict, how in LINE.findall(text):
        if verdict == "ESCAPED":
            bad.append(f"{name}: ESCAPED: {how}")
        seen.setdefault(name, []).append((verdict, how))
    for name in ATTACKS:
        if name not in seen:
            bad.append(f"{name}: never attempted")
        elif not any(v == "refused" and how.strip() for v, how in seen[name]):
            bad.append(f"{name}: no refusal that says how")
    bad += unproven_rows(seen)
    return seen, bad


ROW_LINES = """cap-escape refused: a socket through net.sockets without Network, MkIpcSend EPERM (1)
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
service-squat refused: MkServiceRegister app.file_manager EPERM (1)"""
FAULTED = """[ATTACK-FAULT] pid 41 writes address 0x0
[TRAP PF] cpl=3 rip=0x0000000000402832 rsp=0x00007fffffffe000 cs=0x0000000000000033 ss=0x000000000000002b rflags=0x0000000000010246 cr3=0x0000000001234000 asid=0x0000000000000009 pid=0x0000000000000029 err=0x0000000000000006 cr2=0x0000000000000000
[TRAP WALK] l4=0
[NET] dhcp lease renewed
"""


def self_test():
    rows = "".join(f"[ATTACK] {r}\n" for r in ROW_LINES.splitlines())
    full = rows + "".join(f"[ATTACK] {a} refused: errno -1\n" for a in ATTACKS
                          if a != "fault-containment") + FAULTED
    assert not judge(full)[1], judge(full)[1]
    assert judge(full.replace("[ATTACK] chainload refused: errno -1\n", ""))[1]
    assert judge(full + "[ATTACK] cap-escape ESCAPED: socket opened\n")[1]
    assert judge(full.replace("chainload refused: errno -1", "chainload refused:"))[1]
    # A matrix row with no refusal of its own fails though its attack passed.
    assert judge(full.replace("register net.dns without RegisterService", "register"))[1]
    assert judge(full.replace("MkPioGrant", "MkPio"))[1]
    assert judge(full.replace("inbox by its pid", "inbox"))[1]
    assert judge(full.replace("driver by its pid", "driver"))[1]
    assert judge(full.replace("a device's driver by name", "a device"))[1]
    # The fault: no trap for that pid, a log that stops, or a kernel fault.
    assert judge(full.replace("pid=0x0000000000000029", "pid=0x000000000000002a"))[1]
    assert judge(full.replace("[NET] dhcp lease renewed\n", ""))[1]
    assert judge(full + "[PANIC PF] fatal kernel fault, no recovery path rip=0x1\n")[1]
    assert not judge(full + "[TRAP PF] cpl=0 rip=0xffff800000100000\n")[1]
    assert judge(full.replace("[ATTACK-FAULT] pid 41 writes address 0x0\n", ""))[1]
    print("attack suite check: self-test PASS")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--log", action="append", default=[], help="a serial log; repeat per boot")
    ap.add_argument("--self-test", action="store_true", help="prove the check bites")
    a = ap.parse_args()
    if a.self_test:
        return self_test() or 0
    if not a.log:
        ap.error("--log is required")
    seen, bad = judge("".join(open(p, errors="replace").read() for p in a.log))
    for name in ATTACKS:
        for verdict, how in seen.get(name, []):
            print(f"  {name}: {verdict}: {how}")
    for b in bad:
        print(f"FAIL {b}")
    print("attack suite: PASS" if not bad else "attack suite: FAIL")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
