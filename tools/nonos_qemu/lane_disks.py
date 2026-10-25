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

"""The lanes with a disk behind their device, one line each: the disk's size,
then the QEMU arguments. {drive} is the disk's -drive value and {id} its id.
Every id starts lane- so a lane never meets one of machine.py's, and every
lane can be added to the same machine."""

# QEMU refuses an SD or eMMC card whose size is not a power of two.
CARD = 4 << 30
STICK = 256 << 20
# The SDHCI capability word: an embedded slot (bits 31:30 = 01), 3.3 V and
# 1.8 V, SDMA, ADMA2 and high speed, a 52 MHz base clock, 512-byte blocks.
# An eMMC host on a laptop's board reports an embedded slot, not a card slot.
EMMC_CAPAREG = 0x457834B4


def usb(host, bus):
    """A USB host with a keyboard on port 1 and a stick on port 2."""
    return (f"{host} -device usb-kbd,bus={bus},port=1 -drive {{drive}} "
            f"-device usb-storage,bus={bus},port=2,drive={{id}},removable=on")


def scsi(host, ident):
    return f"-device {host},id={ident} -drive {{drive}} -device scsi-hd,bus={ident}.0,drive={{id}}"


DISKS = {
    "emmc": (CARD, f"-device sdhci-pci,capareg={EMMC_CAPAREG:#x} -drive {{drive}} -device emmc,drive={{id}}"),
    # A card in a removable slot: the host's default capability word.
    "sd": (CARD, "-device sdhci-pci -drive {drive} -device sd-card,drive={id}"),
    "ufs": (CARD, "-device ufs,serial=NONOS-UFS,id=lane-ufs -drive {drive} -device ufs-lu,drive={id},lun=0,bus=lane-ufs"),
    # EHCI with its three UHCI companions, wired as on an Intel board. The
    # stick runs at high speed on EHCI. EHCI alone cannot reach the
    # full-speed keyboard, so its port is handed to a companion.
    "ehci": (STICK, usb("-device ich9-usb-ehci1,id=lane-ehci,addr=1d.7,multifunction=on"
                        + "".join(f" -device ich9-usb-uhci{n + 1},id=lane-ehci-uhci{n + 1},addr=1d.{n},"
                                  f"multifunction=on,masterbus=lane-ehci.0,firstport={2 * n}" for n in range(3)),
                        "lane-ehci.0")),
    # A stick on the machine's own xHCI controller, beside the boot disk.
    "xhci-stick": (STICK, "-drive {drive} -device usb-storage,bus=xhci.0,drive={id},removable=on"),
    "uhci": (STICK, usb("-device piix3-usb-uhci,id=lane-uhci", "lane-uhci.0")),
    "ohci": (STICK, usb("-device pci-ohci,id=lane-ohci", "lane-ohci.0")),
    "pvscsi": (CARD, scsi("pvscsi", "lane-pvscsi")),
    "lsi53c895a": (CARD, scsi("lsi53c895a", "lane-lsi")),
    "mptsas1068": (CARD, scsi("mptsas1068", "lane-mptsas")),
    "virtio-scsi": (CARD, scsi("virtio-scsi-pci", "lane-vscsi")),
    "piix4-ide": (CARD, "-device piix4-ide,id=lane-ide -drive {drive} -device ide-hd,bus=lane-ide.0,drive={id}"),
}
