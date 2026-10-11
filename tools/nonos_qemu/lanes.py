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

"""Device lanes, opt-in: each adds a device NONOS has or is getting a driver
for, to boot that driver under QEMU. --lane and make's QEMU_LANES both read
this table, so a lane is the same machine either way. Lanes combine."""

import collections
import os

from . import lane_disks, lane_usb

NICS = ("e1000", "e1000e", "igb", "rtl8139")
# lane: (its disk's size, the QEMU lines given that disk's path)
DISKS = {
    "emmc": (lane_disks.CARD_BYTES, lane_disks.emmc),
    "sd": (lane_disks.CARD_BYTES, lane_disks.sd),
    "ufs": (lane_disks.CARD_BYTES, lane_disks.ufs),
    "ehci": (lane_usb.STICK_BYTES, lane_usb.ehci),
    "uhci": (lane_usb.STICK_BYTES, lane_usb.uhci),
    "ohci": (lane_usb.STICK_BYTES, lane_usb.ohci),
}
NAMES = (*DISKS, *(f"nic={n}" for n in NICS), "tpm-tis", "usb-audio")

# args: lines to add. images: (path, bytes) to make. nic and tpm: the model
# that takes the place of the virtio NIC and of tpm-crb, or None.
Plan = collections.namedtuple("Plan", "args images nic tpm")


class LaneError(ValueError):
    pass


def plan(names, root, audio="none"):
    args, images, nic, tpm = [], [], None, None
    # A lane named twice is one lane.
    for name in dict.fromkeys(names):
        if name.startswith("nic=") and name[4:] in NICS:
            if nic:
                raise LaneError(f"nic={nic} and {name}: the machine has one network card")
            nic = name[4:]
        elif name == "tpm-tis":
            tpm = "tpm-tis"
        elif name == "usb-audio":
            args += lane_usb.usb_audio(audio)
        elif name in DISKS:
            size, lines = DISKS[name]
            path = os.path.join(root, f"lane-{name}.img")
            images.append((path, size))
            args += lines(path)
        else:
            raise LaneError(f"no lane {name!r}: the lanes are {', '.join(NAMES)}")
    return Plan(args, images, nic, tpm)


def make_images(images):
    """Blank the first time, then kept, so a second boot sees what a first wrote."""
    for path, size in images:
        if not os.path.exists(path):
            os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
            with open(path, "wb") as f:
                f.truncate(size)
