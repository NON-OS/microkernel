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
  nix run .#qemu -- --lane nic=e1000e --lane emmc
                                          device lanes, to boot a driver (lanes.py)
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

from . import disk, extra, iommu, lanes, machine, siblings, tpm

TARGET = "target/qemu"


def args():
    ap = argparse.ArgumentParser(prog="nonos-qemu", description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--profile", default="", help="the sealed profile to boot (default: the only one sealed)")
    ap.add_argument("--image", help="a USB image to boot instead of a sealed one")
    ap.add_argument("--tpm", action="store_true", help="attach a software TPM")
    ap.add_argument("--fresh", action="store_true", help="start the data disk again from the sealed image")
    ap.add_argument("--keep-machine", action="store_true",
                    help="boot on the last boot's EFI variable store and system partition as it left them")
    ap.add_argument("--stick", action="store_true", help="boot the sealed stick alone, with no data volume")
    ap.add_argument("--usb", action="store_true",
                    help="with --stick, plug the stick into the xHCI controller as a USB mass-storage device")
    ap.add_argument("--model", action="append", default=[], metavar="TIER",
                    help="lay a Qwen tier on a new data disk: a tier nonos-qwen-tier.py list names, "
                         "or auto for the one setup picks for --mem")
    ap.add_argument("--smp", type=int, default=0, help="CPUs (default: 8, or this machine's cores when fewer)")
    ap.add_argument("--mem", default="8G", help="memory: Qwen, Linux programs and a second window need more than 2G")
    ap.add_argument("--net", choices=["nat", "off"], default="nat")
    ap.add_argument("--capture", metavar="PCAP",
                    help="write every frame the guest sends to this capture, and what it is sent to "
                         "PCAP-in.pcap beside it (tools/nonos-pcap-egress reads both)")
    ap.add_argument("--install-target", action="store_true", help="attach a blank NVMe disk to install to")
    ap.add_argument("--installed", action="store_true", help="boot the NVMe disk the installer wrote, alone")
    ap.add_argument("--lane", action="append", default=[], metavar="LANE",
                    help="add a device lane: " + ", ".join(lanes.NAMES))
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
    """Waits for every pattern on the serial console. With none, a boot still running at the
    timeout passes; one that stopped did not, nor one whose capsules did not all spawn."""
    deadline = time.time() + timeout
    pending = [re.compile(p) for p in expect]
    while time.time() < deadline:
        time.sleep(2)
        text = open(serial, errors="replace").read() if os.path.exists(serial) else ""
        if "[INIT] capsule spawn incomplete" in text:
            return print("the serial console says [INIT] capsule spawn incomplete") or 1
        pending = [p for p in pending if not p.search(text)]
        if expect and not pending:
            return 0
        if proc.poll() is not None:
            return print(f"QEMU stopped (exit {proc.returncode}) before the boot finished") or 1
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
            disk.prepare(source, *boot, a.fresh, disk.resolve(a.model, a.mem), a.keep_machine)
    target = os.path.join(TARGET, "install-target.img")
    if a.install_target and not os.path.exists(target):
        with open(target, "wb") as f:
            f.truncate(machine.INSTALL_TARGET_GB << 30)
    if a.installed and not os.path.exists(target):
        sys.exit("no install target yet: boot with --install-target and install first")
    if a.usb and (not a.stick or a.installed):
        sys.exit("--usb plugs in the sealed stick: give it with --stick")
    try:
        lane = lanes.plan(a.lane, TARGET, machine.audio())
    except lanes.LaneError as e:
        sys.exit(str(e))
    if a.keep_machine and (a.fresh or a.stick or a.installed):
        sys.exit("--keep-machine boots on the last boot's data disk, partition and variables: not with --fresh, --stick or --installed")
    if a.capture and a.net == "off":
        sys.exit("--capture records the network, and --net off has none")
    capture = machine.capture(a.capture) if a.capture else []
    if ("nic" in lane.swaps and a.net == "off") or ("tpm" in lane.swaps and not a.tpm):
        sys.exit("a nic= lane needs the network on, and tpm-tis needs --tpm: each swaps a device in")
    lanes.make_images(lane.images)
    vars_copy = disk.vars_store(TARGET, os.environ["OVMF_VARS"], a.keep_machine)
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
           *machine.devices(a.net, sock, os.path.join(TARGET, "qmp.sock"), lane.swaps), *capture, *lane.args,
           *(machine.usb_stick(stick) if a.usb else []),
           *serial, *machine.display(a.headless), "-no-reboot", *extra.words()]
    print(" ".join(cmd := iommu.apply(siblings.apply(cmd, lane.swaps), lane.swaps.get("iommu"))), flush=True)
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
