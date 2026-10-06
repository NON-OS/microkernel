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

"""nix run .#qemu [-- options]: boot the image the seal wrote.

  nix run .#qemu                          the sealed image, in a window
  nix run .#qemu -- --tpm                 with a software TPM (measured boot)
  nix run .#qemu -- --fresh --model auto  a new data disk carrying the Qwen tier
                                          setup picks for the memory (--model
                                          small, qwen3-4b ... names one), each
                                          imported on its first use
  nix run .#qemu -- --stick               the sealed stick alone, as hardware
                                          boots it (no data volume)
  nix run .#qemu -- --stick --usb         that stick on a USB port, served by
                                          driver.xhci0 and driver.usb_msc0
  The data disk is kept between boots, and the encrypted volume with it; a
  new seal or --fresh starts it again from the sealed image.
  nix run .#qemu -- --install-target      with a blank NVMe disk to install to
  nix run .#qemu -- --installed           the installed disk alone
  nix run .#qemu -- --headless --timeout 300 --expect '\\[BOOT-ATTEST\\].*enrolled'
                                          a boot smoke: passes when every
                                          pattern is on the serial console"""

import argparse
import os
import re
import shutil
import subprocess
import sys
import time

from . import disk, machine, tpm

TARGET = "target/qemu"


def args():
    ap = argparse.ArgumentParser(prog="nonos-qemu", description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--profile", default="", help="the sealed profile to boot (default: the only one sealed)")
    ap.add_argument("--image", help="a USB image to boot instead of a sealed one")
    ap.add_argument("--tpm", action="store_true", help="attach a software TPM")
    ap.add_argument("--fresh", action="store_true", help="start the data disk again from the sealed image")
    ap.add_argument("--stick", action="store_true", help="boot the sealed stick alone, with no data volume")
    ap.add_argument("--usb", action="store_true",
                    help="with --stick, plug the stick into the xHCI controller as a USB mass-storage device")
    ap.add_argument("--model", action="append", default=[], metavar="TIER",
                    help="lay a Qwen tier on a new data disk: a tier nonos-qwen-tier.py list names, "
                         "or auto for the one setup picks for --mem")
    ap.add_argument("--smp", type=int, default=0, help="CPUs (default: 8, or this machine's cores when fewer)")
    ap.add_argument("--mem", default="8G", help="memory: Qwen, Linux programs and a second window need more than 2G")
    ap.add_argument("--net", choices=["nat", "off"], default="nat")
    ap.add_argument("--install-target", action="store_true", help="attach a blank NVMe disk to install to")
    ap.add_argument("--installed", action="store_true", help="boot the NVMe disk the installer wrote, alone")
    ap.add_argument("--headless", action="store_true", help="no window; serial to --serial")
    ap.add_argument("--serial", default=f"{TARGET}/serial.log")
    ap.add_argument("--timeout", type=int, default=0, help="seconds before a headless boot is stopped")
    ap.add_argument("--expect", action="append", default=[], help="a pattern the serial console must show")
    return ap.parse_args()


def sealed_image(profile):
    root = "target/release"
    names = sorted(n for n in os.listdir(root) if os.path.isfile(os.path.join(root, n, "nonos.img"))) \
        if os.path.isdir(root) else []
    # "default" is nonos.toml's profile, sealed under its own name: the one
    # image there is, as with no profile.
    if profile and profile != "default":
        names = [n for n in names if n == profile or n.startswith(profile + "-")]
    if len(names) != 1:
        sys.exit(f"no single sealed image under {root} ({', '.join(names) or 'none'}): nix run .#seal, or pass --image")
    return os.path.join(root, names[0], "nonos.img")


def watch(proc, serial, timeout, expect):
    """Waits for every pattern on the serial console. With none, a boot that
    is still running at the timeout passes; one that stopped did not."""
    deadline = time.time() + timeout
    pending = [re.compile(p) for p in expect]
    while time.time() < deadline:
        time.sleep(2)
        with open(serial, errors="replace") if os.path.exists(serial) else open(os.devnull) as f:
            text = f.read()
        pending = [p for p in pending if not p.search(text)]
        if expect and not pending:
            return 0
        if proc.poll() is not None:
            print(f"QEMU stopped (exit {proc.returncode}) before the boot finished")
            return 1
    if pending:
        print("not on the serial console after %ds: %s" % (timeout, ", ".join(p.pattern for p in pending)))
        return 1
    return 0


def main():
    a = args()
    os.makedirs(TARGET, exist_ok=True)
    stick = os.path.join(TARGET, "stick.img")
    boot = stick
    if not a.installed:
        source = a.image or sealed_image(a.profile)
        if a.stick:
            shutil.copyfile(source, stick)
        else:
            boot = (os.path.join(TARGET, "esp"), os.path.join(TARGET, "data.img"))
            disk.prepare(source, *boot, a.fresh, disk.resolve(a.model, a.mem))
    target = os.path.join(TARGET, "install-target.img")
    if a.install_target and not os.path.exists(target):
        with open(target, "wb") as f:
            f.truncate(machine.INSTALL_TARGET_GB << 30)
    if a.installed and not os.path.exists(target):
        sys.exit("no install target yet: boot with --install-target and install first")
    if a.usb and (not a.stick or a.installed):
        sys.exit("--usb plugs in the sealed stick: give it with --stick")
    vars_copy = os.path.join(TARGET, "OVMF_VARS.fd")
    shutil.copyfile(os.environ["OVMF_VARS"], vars_copy)
    os.chmod(vars_copy, 0o644)
    # One TPM for this machine, kept across boots; a new one with a new data disk.
    swtpm, sock, made = tpm.start(os.path.abspath(os.path.join(TARGET, "swtpm")), a.fresh) \
        if a.tpm else (None, None, False)
    kept = [d for d in (os.path.join(TARGET, "data.img"), target) if os.path.exists(d)]
    if made and kept and not a.fresh:
        print("note: this is a new TPM, so a data volume an earlier boot made on "
              f"{', '.join(kept)} will not open; FRESH=1 starts the data disk again, and "
              "deleting the install target lets the installer make a new one", flush=True)
    # The volume key is bound to the kernel the loader measured (PCR 9): an image
    # sealed since opens no volume an older one made, by design.
    serial = ["-serial", f"file:{a.serial}"] if a.headless else ["-serial", "mon:stdio"]
    cmd = ["qemu-system-x86_64", "-machine", "q35", "-m", a.mem, "-smp", str(machine.cpus(a.smp)), *machine.accel(),
           *machine.firmware(os.environ["OVMF"], vars_copy),
           *machine.disks(boot, target if (a.install_target or a.installed) else None, a.installed, a.usb),
           *machine.devices(a.net, sock, os.path.join(TARGET, "qmp.sock")),
           *(machine.usb_stick(stick) if a.usb else []),
           *serial, *machine.display(a.headless), "-no-reboot"]
    print(" ".join(cmd), flush=True)
    proc = None
    try:
        proc = subprocess.Popen(cmd)
        rc = watch(proc, a.serial, a.timeout, a.expect) if a.headless and a.timeout else proc.wait()
    finally:
        if proc and proc.poll() is None:
            proc.terminate()
        if swtpm:
            swtpm.terminate()
    sys.exit(rc)


if __name__ == "__main__":
    main()
