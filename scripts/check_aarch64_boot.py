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
"""Boot the aarch64 kernel under QEMU virt and hold its serial log to one cell's claims.

core               the core profile reaches its named markers in order, and no
                   trap, refusal or panic line appears on the way
trap-sp0           built with nonos-trap-proof-sp0: exactly one [TRAP] line for
                   the SP_EL0 vector, carrying the syndrome of `brk #0x5350`
                   (ESR 0xF2005350) and no FAR, which a breakpoint does not
                   write, a saved SPSR that says EL1 on SP_EL0, the SP_EL0 value
                   the proof loaded, and an ELR that points at that very
                   instruction in the image
trap-kernel-abort  built with nonos-trap-proof-kernel-abort: exactly two [TRAP]
                   lines, a kernel page fault whose ESR is a same-EL data abort
                   with a level 0 translation fault on a read, and the faulting
                   address with its access, both as the trap contract prints them

A trap cell also fails if any kernel line follows the last [TRAP] line, or if
the boot reaches Core ready. QEMU is stopped as soon
as the cell has what it needs plus a short settle, or at the deadline. The log
is written whole for the lane to upload. --replay judges a saved log without
QEMU; --self-test proves each cell rejects the logs it must reject.
"""

import argparse
import re
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from check_kernel_elf import Elf, PF_X, PT_LOAD  # noqa: E402

QEMU_FLAGS = ["-M", "virt,gic-version=3", "-cpu", "max", "-m", "512", "-nographic",
              "-serial", "mon:stdio", "-device", "virtio-rng-pci"]
SETTLE_SECS = 3.0

CORE_MARKERS = ["[KSEC] 4/4 sections mapped as declared", "[NONOS] Core ready",
                "[UKERNEL] Entering userspace", "[INIT] Starting"]
NEVER = ["[TRAP]", "KERNEL FATAL TRAP", "[BOOT] refused", "KERNEL PANIC"]

HEX = r"0x([0-9A-F]{16})"
SP0_LINE = re.compile(rf"^\[TRAP\] SP_EL0 vector sync esr={HEX} elr={HEX} spsr={HEX} sp={HEX}$")
SP0_ESR = 0xF2005350
SP0_MARK = 0x0000535053505350
BRK_5350 = (0xD4200000 | (0x5350 << 5)).to_bytes(4, "little")
SPSR_M_EL1T = 0b0100

ABORT_LINE = re.compile(rf"^\[TRAP\] KERNEL FATAL TRAP: Page Fault esr={HEX} elr={HEX} sp={HEX} origin=EL1$")
ABORT_DETAIL = "[TRAP] far=0x0000400000000000 read not-present EL1"
EC_DATA_ABORT_SAME = 0x25
DFSC_TRANSLATION_L0 = 0x04
WNR = 1 << 6


def trap_lines(lines):
    return [line for line in lines if line.startswith("[TRAP]")]


def in_text(elf, addr):
    return elf is None or any(
        t == PT_LOAD and f & PF_X and v <= addr < v + m for t, f, v, m in elf.segments())


def judge_core(lines, elf):
    failed = []
    at = 0
    for marker in CORE_MARKERS:
        hits = [i for i, line in enumerate(lines) if marker in line]
        if not hits:
            failed.append(f"marker never printed: {marker}")
        elif hits[0] < at:
            failed.append(f"marker out of order: {marker}")
        else:
            at = hits[0]
    for bad in NEVER:
        for line in lines:
            if bad in line:
                failed.append(f"forbidden line: {line}")
    return failed


def judge_sp0(lines, elf):
    failed = []
    traps = trap_lines(lines)
    if len(traps) != 1:
        failed.append(f"want exactly one [TRAP] line, got {len(traps)}")
    match = next((SP0_LINE.match(t) for t in traps if SP0_LINE.match(t)), None)
    if not match:
        return failed + ["no line of the form `[TRAP] SP_EL0 vector sync esr=.. elr=.. spsr=.. sp=..`"]
    esr, elr, spsr, sp = (int(g, 16) for g in match.groups())
    if esr != SP0_ESR:
        failed.append(f"esr {esr:#x}, want {SP0_ESR:#x} (EC 0x3C, IL, imm 0x5350)")
    if spsr & 0xF != SPSR_M_EL1T:
        failed.append(f"saved SPSR.M {spsr & 0xF:#06b}, want {SPSR_M_EL1T:#06b} (EL1 on SP_EL0)")
    if sp != SP0_MARK:
        failed.append(f"sp {sp:#x}, want the SP_EL0 the proof loaded, {SP0_MARK:#x}")
    if not in_text(elf, elr):
        failed.append(f"elr {elr:#x} lies in no executable segment of the image")
    elif elf is not None and elf.read(elr, 4) != BRK_5350:
        failed.append(f"the instruction at elr {elr:#x} is not `brk #0x5350`")
    return failed + ran_on(lines)


def judge_abort(lines, elf):
    failed = []
    traps = trap_lines(lines)
    if len(traps) != 2:
        failed.append(f"want exactly two [TRAP] lines, got {len(traps)}")
    match = next((ABORT_LINE.match(t) for t in traps if ABORT_LINE.match(t)), None)
    if not match:
        return failed + ["no line of the form `[TRAP] KERNEL FATAL TRAP: Page Fault esr=.. elr=.. sp=.. origin=EL1`"]
    esr, elr, _ = (int(g, 16) for g in match.groups())
    if (esr >> 26) & 0x3F != EC_DATA_ABORT_SAME:
        failed.append(f"esr {esr:#x} has EC {(esr >> 26) & 0x3F:#x}, want {EC_DATA_ABORT_SAME:#x} (data abort, same EL)")
    if esr & 0x3F != DFSC_TRANSLATION_L0:
        failed.append(f"esr {esr:#x} has DFSC {esr & 0x3F:#x}, want {DFSC_TRANSLATION_L0:#x} (level 0 translation)")
    if esr & WNR:
        failed.append(f"esr {esr:#x} says write, the proof reads")
    if not in_text(elf, elr):
        failed.append(f"elr {elr:#x} lies in no executable segment of the image")
    if traps.count(ABORT_DETAIL) != 1 or traps.index(ABORT_DETAIL) != traps.index(match.group(0)) + 1:
        failed.append(f"the report is not followed by `{ABORT_DETAIL}`")
    return failed + ran_on(lines)


def ran_on(lines):
    """A terminal trap parks the CPU, so no kernel line may follow the last one."""
    failed = [f"the boot reached Core ready: {line}" for line in lines if "[NONOS] Core ready" in line]
    last = max((i for i, line in enumerate(lines) if line.startswith("[TRAP]")), default=len(lines))
    return failed + [f"the boot ran past the trap: {line}" for line in lines[last + 1:]
                     if line.strip() and not line.startswith("qemu-system-")]


CELLS = {
    "core": (judge_core, lambda lines: "[INIT] Starting" in "\n".join(lines)),
    "trap-sp0": (judge_sp0, lambda lines: any(SP0_LINE.match(t) for t in trap_lines(lines))),
    "trap-kernel-abort": (judge_abort, lambda lines: ABORT_DETAIL in lines),
}


def boot(qemu, elf_path, cell, deadline, log_path):
    """Run QEMU until the cell is satisfied plus a settle, or the deadline."""
    proc = subprocess.Popen([qemu, *QEMU_FLAGS, "-kernel", str(elf_path)],
                            stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT)
    lines = []
    reader = threading.Thread(target=lambda: lines.extend(
        raw.decode("utf-8", "replace").rstrip("\r\n") for raw in proc.stdout), daemon=True)
    reader.start()
    done = CELLS[cell][1]
    started = time.monotonic()
    satisfied_at = None
    while time.monotonic() - started < deadline and proc.poll() is None:
        if satisfied_at is None and done(list(lines)):
            satisfied_at = time.monotonic()
        if satisfied_at is not None and time.monotonic() - satisfied_at >= SETTLE_SECS:
            break
        time.sleep(0.2)
    proc.kill()
    proc.wait()
    reader.join(timeout=5)
    elapsed = time.monotonic() - started
    log_path.write_text("\n".join(lines) + "\n")
    return lines, elapsed, satisfied_at is not None


def self_test():
    good_sp0 = ["[NONOS] dtb at 0x0000000000000000",
                "[TRAP] SP_EL0 vector sync esr=0x00000000F2005350 "
                "elr=0x0000000040110110 spsr=0x00000000600003C4 sp=0x0000535053505350"]
    good_abort = ["[TRAP] KERNEL FATAL TRAP: Page Fault esr=0x0000000096000004 "
                  "elr=0x00000000400C6D5C sp=0x000000004140FB80 origin=EL1", ABORT_DETAIL]
    good_core = ["[KSEC] 4/4 sections mapped as declared", "[NONOS] Core ready",
                 "[UKERNEL] Entering userspace", "[INIT] Starting"]
    cases = [
        ("core", good_core, True),
        ("core", good_core[:3], False),
        ("core", [good_core[1], good_core[0]] + good_core[2:], False),
        ("core", good_core + ["[BOOT] refused: gic: x, mpidr=0x0000000000000000"], False),
        ("core", good_core + ["!!! KERNEL PANIC !!!"], False),
        ("trap-sp0", good_sp0, True),
        ("trap-sp0", [good_sp0[1].replace("F2005350", "F2005351")], False),
        ("trap-sp0", [good_sp0[1].replace("600003C4", "600003C5")], False),
        ("trap-sp0", good_sp0 + good_sp0[1:], False),
        ("trap-sp0", good_sp0 + ["[NONOS] Core ready"], False),
        ("trap-sp0", [good_sp0[1].replace("0x0000535053505350", "0x000000004140FB80")], False),
        ("trap-sp0", [good_sp0[1].replace(" elr=", " far=0x0000000000000000 elr=")], False),
        ("trap-sp0", good_sp0 + ["[NONOS] no usable device tree, assuming QEMU virt"], False),
        ("trap-sp0", good_sp0 + ["qemu-system-aarch64: terminating on signal 15"], True),
        ("trap-kernel-abort", good_abort, True),
        ("trap-kernel-abort", [good_abort[0].replace("96000004", "96000005"), ABORT_DETAIL], False),
        ("trap-kernel-abort", [good_abort[0].replace("96000004", "96000044"), ABORT_DETAIL], False),
        ("trap-kernel-abort", [good_abort[0].replace("96000004", "92000004"), ABORT_DETAIL], False),
        ("trap-kernel-abort", good_abort[:1], False),
        ("trap-kernel-abort", [ABORT_DETAIL, good_abort[0]], False),
        ("trap-kernel-abort", good_abort + ["[TRAP-PROOF] read of an unmapped address returned 0x0"], False),
    ]
    wrong = [(cell, i) for i, (cell, log, ok) in enumerate(cases)
             if (not CELLS[cell][0](log, None)) != ok]
    if wrong:
        print(f"aarch64-boot: self-test failed on cases {wrong}")
        return 1
    print(f"aarch64-boot: self-test passed, {sum(not ok for *_, ok in cases)} bad logs rejected "
          f"and {sum(ok for *_, ok in cases)} good ones accepted")
    return 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--cell", choices=sorted(CELLS))
    ap.add_argument("--elf", type=Path, help="the kernel ELF to boot and to read ELR against")
    ap.add_argument("--log", type=Path, help="where the serial log is written")
    ap.add_argument("--deadline", type=float, default=300.0, help="seconds before QEMU is stopped")
    ap.add_argument("--qemu", default="qemu-system-aarch64")
    ap.add_argument("--replay", type=Path, help="judge this saved log instead of booting")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    if not a.cell or not a.elf:
        ap.error("--cell and --elf are required")
    elf = Elf(a.elf.read_bytes())
    if a.replay:
        lines = a.replay.read_text(errors="replace").splitlines()
        note = f"replayed {a.replay}"
    else:
        log = a.log or Path(tempfile.mkdtemp()) / f"{a.cell}.log"
        lines, elapsed, satisfied = boot(a.qemu, a.elf, a.cell, a.deadline, log)
        note = f"{len(lines)} lines in {elapsed:.0f}s, log {log}" + ("" if satisfied else ", deadline hit")
    failed = CELLS[a.cell][0](lines, elf)
    if failed:
        print(f"aarch64-boot: cell {a.cell} FAILED ({note}):")
        for line in failed:
            print(f"  {line}")
        return 1
    print(f"aarch64-boot: cell {a.cell} passed ({note})")
    for line in trap_lines(lines):
        print(f"  {line}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
