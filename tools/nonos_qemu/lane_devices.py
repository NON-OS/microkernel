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

"""The lanes with no disk, one line each. A plain lane adds its QEMU
arguments; {audio} is the audio backend, and each sound lane names an
audiodev of its own, since a make target may define none. A swap lane puts
its model in place of one of machine.py's devices: the virtio NIC, tpm-crb
or the virtio GPU (vmware-svga, gpu=bga, gpu=gop-only)."""

PLAIN = {
    "usb-audio": "-audiodev {audio},id=lane-usb-snd -device usb-audio,audiodev=lane-usb-snd,bus=xhci.0",
    "AC97": "-audiodev {audio},id=lane-ac97-snd -device AC97,audiodev=lane-ac97-snd",
    # A reader with no card in it: driver.ccid0 finds the slot empty.
    "usb-ccid": "-device usb-ccid,bus=xhci.0",
    "usb-tablet": "-device usb-tablet,bus=xhci.0,id=lane-usb-tablet",
    "usb-kbd": "-device usb-kbd,bus=xhci.0,id=lane-usb-kbd",
    "virtio-tablet": "-device virtio-tablet-pci",
    "virtio-balloon": "-device virtio-balloon-pci",
    "vmbus": "-device vmbus-bridge -device hv-balloon",
    # Not a device: the machine stops at power-off instead of exiting, so a
    # test can read its memory after the wipe. The launcher's -no-reboot
    # makes a restart a power-off, so a restart stops there too.
    "hold-at-off": "-action shutdown=pause",
}

NICS = ("e1000", "e1000e", "igb", "rtl8139", "pcnet", "vmxnet3", "tulip")

# lane: (the device it takes the place of, its model)
SWAPS = {
    **{f"nic={n}": ("nic", n) for n in NICS},
    "tpm-tis": ("tpm", "tpm-tis"),
    "vmware-svga": ("gpu", "vmware-svga"),
    "bochs-display": ("gpu", "bochs-display"),
    # QEMU's usb-net offers RNDIS and CDC-ECM configurations, on the xHCI the
    # machine always has; as net0 its frames are the capture's.
    "nic=usb-net": ("nic", "usb-net,bus=xhci.0"),
    # QEMU's -vga std: the Bochs display (1234:1111) driver.bga is for.
    "gpu=bga": ("gpu", "VGA"),
    # No PCI display at all: OVMF's GOP over ramfb, the boot framebuffer alone.
    "gpu=gop-only": ("gpu", "ramfb"),
    # Not a device in place of another: iommu.py rewrites the machine for it.
    "iommu=intel": ("iommu", "intel"),
    "iommu=amd": ("iommu", "amd"),
    # Neither: siblings.py moves the HDA and the NIC behind one root port.
    "hda-beside-nic": ("audio", "beside-nic"),
}
