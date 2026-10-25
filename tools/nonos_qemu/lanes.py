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
these tables, so a lane is the same machine either way. A new lane is one
line in lane_disks.py or lane_devices.py. Lanes combine."""

import collections
import os

from .lane_devices import PLAIN, SWAPS
from .lane_disks import DISKS

NAMES = (*DISKS, *PLAIN, *SWAPS)

# args: lines to add. images: (path, bytes) to make. swaps: device kind
# ("nic", "tpm", "gpu") to the model that takes its place.
Plan = collections.namedtuple("Plan", "args images swaps")


class LaneError(ValueError):
    pass


def lines(template, **values):
    # Split before filling in, so a path with a space stays one argument.
    return [word.format(**values) for word in template.split()]


def plan(names, root, audio="none"):
    args, images, swaps = [], [], {}
    # A lane named twice is one lane.
    for name in dict.fromkeys(names):
        if name in SWAPS:
            kind, model = SWAPS[name]
            if swaps.get(kind, model) != model:
                raise LaneError(f"{kind} {swaps[kind]} and {model}: the machine has one {kind}")
            swaps[kind] = model
        elif name in PLAIN:
            args += lines(PLAIN[name], audio=audio)
        elif name in DISKS:
            size, template = DISKS[name]
            path = os.path.join(root, f"lane-{name}.img")
            images.append((path, size))
            ident = f"lane-{name}-disk"
            args += lines(template, drive=f"file={path},if=none,id={ident},format=raw", id=ident)
        else:
            raise LaneError(f"no lane {name!r}: the lanes are {', '.join(NAMES)}")
    if "audio" in swaps and "nic" not in swaps:
        raise LaneError("hda-beside-nic needs a nic= lane: the NIC is the function beside the HDA")
    return Plan(args, images, swaps)


def make_images(images):
    """Blank the first time, then kept, so a second boot sees what a first wrote."""
    for path, size in images:
        if not os.path.exists(path):
            os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
            with open(path, "wb") as f:
                f.truncate(size)

