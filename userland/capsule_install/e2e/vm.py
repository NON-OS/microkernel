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
"""The QEMU command line the e2e runs use, the same machine the make run
targets start: q35, OVMF, virtio-vga, virtio-rng, xhci, and a swtpm TPM 2.0
on the CRB interface when asked."""

import os
import shutil
import subprocess
import time

OVMF = "/usr/share/OVMF/OVMF_CODE_4M.fd"


def start_swtpm(out):
    state = os.path.join(out, "swtpm")
    shutil.rmtree(state, ignore_errors=True)
    os.makedirs(state)
    sock = os.path.join(state, "sock")
    subprocess.Popen(["swtpm", "socket", "--tpm2", "--tpmstate", f"dir={state}",
                      "--ctrl", f"type=unixio,path={sock}", "--flags", "startup-clear",
                      "--log", f"file={state}/swtpm.log", "--terminate"])
    for _ in range(50):
        if os.path.exists(sock):
            return sock
        time.sleep(0.1)
    raise RuntimeError("swtpm did not start")


def qemu_cmd(a, serial, qmp, tpm):
    cmd = ["qemu-system-x86_64", "-m", a.mem, "-accel", "tcg", "-cpu", "max", "-smp", "1",
           "-machine", "q35", "-no-reboot", "-vga", "none",
           "-display", "sdl,show-cursor=off" if getattr(a, "window", False) else "none",
           "-drive", f"if=pflash,format=raw,unit=0,readonly=on,file={OVMF}",
           "-drive", f"if=pflash,format=raw,unit=1,file={a.vars}",
           "-serial", f"file:{serial}", "-qmp", f"unix:{qmp},server=on,wait=off",
           "-device", "virtio-rng-pci", "-device", "qemu-xhci,id=xhci"]
    gpu = "virtio-vga,disable-modern=on,vectors=0,edid=on"
    if a.res:
        w, h = a.res.split("x")
        gpu += f",xres={w},yres={h}"
        if int(w) > 2560:
            gpu = f"VGA,vgamem_mb=64,xres={w},yres={h}"
    cmd += ["-device", gpu]
    if a.esp:
        cmd += ["-drive", f"format=raw,file=fat:rw:{a.esp}"]
    for i, d in enumerate(a.disk):
        path, _, bus = d.partition(",")
        cmd += ["-drive", f"file={path},if=none,id=d{i},format=raw"]
        dev = "nvme,serial=NONOS-TARGET" if bus == "nvme" else "virtio-blk-pci"
        cmd += ["-device", f"{dev},drive=d{i}"]
    if tpm:
        cmd += ["-chardev", f"socket,id=chrtpm,path={tpm}", "-tpmdev",
                "emulator,id=tpm0,chardev=chrtpm", "-device", "tpm-crb,tpmdev=tpm0"]
    return cmd
