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

"""The QEMU command line, one device at a time. These are the devices
mk/10-qemu.mk always gave the image."""

import os
import platform

INSTALL_TARGET_GB = 8


def accel():
    """KVM where /dev/kvm opens read-write, the hypervisor framework on an
    Intel Mac, the emulator otherwise: Apple silicon's hypervisor runs only
    arm64 guests. Under TCG the CPU is emulated, so it is `max`."""
    if os.access("/dev/kvm", os.R_OK | os.W_OK):
        return ["-accel", "kvm", "-cpu", "host,+rdrand,+rdseed"]
    if platform.system() == "Darwin" and platform.machine() == "x86_64":
        return ["-accel", "hvf", "-cpu", "host,+rdrand,+rdseed"]
    # Emulated: one host thread per guest CPU, and a translation cache large
    # enough that a model's inner loops stay translated.
    return ["-accel", "tcg,thread=multi,tb-size=1024", "-cpu", "max"]


def cpus(asked):
    """The guest's CPUs: as asked, else eight, or every core this machine has
    when it has fewer. Every profile is built with SMP on, and a model or a
    proof uses every core it is given. Emulated, each guest CPU runs on its
    own host thread, so more of them is the largest speed-up there."""
    if asked > 0:
        return asked
    return max(1, min(8, os.cpu_count() or 1))


def display(headless):
    if headless:
        return ["-display", "none"]
    ui = "cocoa" if platform.system() == "Darwin" else "gtk"
    return ["-vga", "none", "-display", f"{ui},zoom-to-fit=on"]


def firmware(code, vars_copy):
    return ["-drive", f"if=pflash,format=raw,unit=0,readonly=on,file={code}",
            "-drive", f"if=pflash,format=raw,unit=1,file={vars_copy}"]


def disks(boot, target, installed, usb=False):
    """What the image boots from: the stick alone, or (a pair) the ESP as a
    FAT drive with the virtio data disk; and the NVMe disk the installer
    writes to. A boot of the installed system attaches the NVMe disk alone.
    With `usb` the stick is left to `usb_stick`, plugged into the xHCI
    controller as real hardware sees it."""
    if installed or (usb and not isinstance(boot, tuple)):
        args = []
    elif isinstance(boot, tuple):
        esp_dir, data = boot
        args = ["-drive", f"format=raw,file=fat:rw:{esp_dir}",
                "-drive", f"file={data},if=none,id=vd0,format=raw", "-device", "virtio-blk-pci,drive=vd0"]
    else:
        args = ["-drive", f"format=raw,file={boot}"]
    if target:
        args += ["-drive", f"file={target},if=none,id=tgt,format=raw", "-device", "nvme,drive=tgt,serial=NONOS-TARGET"]
    return args


def usb_stick(path, block_size=512):
    """A USB mass-storage stick on the xHCI controller `devices` adds, first
    to boot: driver.xhci0 and driver.usb_msc0 serve it, not a disk driver of
    the kernel's own. A `block_size` past 512 gives it logical blocks of that
    many bytes, as a 4Kn drive in a USB case has."""
    stick = "usb-storage,bus=xhci.0,drive=usbstick,removable=on,bootindex=0"
    if block_size != 512:
        stick += f",logical_block_size={block_size},physical_block_size={block_size}"
    return ["-drive", f"file={path},if=none,id=usbstick,format=raw", "-device", stick]


def devices(net, tpm_sock, qmp):
    args = ["-device", "virtio-vga,disable-modern=on,vectors=0,edid=on,xres=1920,yres=1080",
            "-device", "qemu-xhci,id=xhci", "-device", "virtio-rng-pci"]
    if net == "nat":
        args += ["-device", "virtio-net-pci,netdev=net0", "-netdev", "user,id=net0"]
    if tpm_sock:
        args += ["-chardev", f"socket,id=chrtpm,path={tpm_sock}", "-tpmdev", "emulator,id=tpm0,chardev=chrtpm",
                 "-device", "tpm-crb,tpmdev=tpm0"]
    audio = "coreaudio" if platform.system() == "Darwin" else "none"
    args += ["-audiodev", f"{audio},id=snd0", "-device", "intel-hda", "-device", "hda-duplex,audiodev=snd0"]
    if qmp:
        args += ["-qmp", f"unix:{qmp},server,nowait"]
    return args
