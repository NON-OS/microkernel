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

"""The disk lanes' QEMU lines: the host a block driver serves and the disk
behind it. Each id starts lane- so a lane never meets one of machine.py's,
and every lane can be added to the same machine."""

# QEMU refuses an SD or eMMC card whose size is not a power of two.
CARD_BYTES = 4 << 30
# The SDHCI capability word: an embedded slot (bits 31:30 = 01), 3.3 V and
# 1.8 V, SDMA, ADMA2 and high speed, a 52 MHz base clock, 512-byte blocks.
# An eMMC host on a laptop's board reports an embedded slot, not a card slot.
EMMC_CAPAREG = 0x457834B4


def drive(path, ident):
    return ["-drive", f"file={path},if=none,id={ident},format=raw"]


def emmc(path):
    return ["-device", f"sdhci-pci,capareg={EMMC_CAPAREG:#x}", *drive(path, "lane-emmc"),
            "-device", "emmc,drive=lane-emmc"]


def sd(path):
    """A card in a removable slot: the host's default capability word."""
    return ["-device", "sdhci-pci", *drive(path, "lane-sd"), "-device", "sd-card,drive=lane-sd"]


def ufs(path):
    return ["-device", "ufs,serial=NONOS-UFS,id=lane-ufs", *drive(path, "lane-ufs-lu0"),
            "-device", "ufs-lu,drive=lane-ufs-lu0,lun=0,bus=lane-ufs"]
