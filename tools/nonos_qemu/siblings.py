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

"""The hda-beside-nic lane: the machine's HDA controller and its NIC as
functions 0 and 1 of one device behind a PCIe root port, as an Intel PCH
puts HDA (1f.3) beside its I219 LAN (1f.6). Behind a root port neither is
a root complex integrated endpoint and neither has ACS, so the two share an
isolation group as on the PCH (HANDOFF/hardware/platform.md), and the
broker gives it to one capsule. Give it a nic= lane, so the NIC beside the
HDA is an Intel model rather than virtio. It moves devices machine.py
already makes, so like iommu.py it works on the finished command line,
and before iommu.py, whose unit must come before the root port."""

KINDS = ("beside-nic",)
PORT = "pcie-root-port,id=lane-siblings,chassis=1,slot=1"
HDA = ",bus=lane-siblings,addr=0.0,multifunction=on"
NIC = ",bus=lane-siblings,addr=0.1"


def apply(cmd, swaps):
    """The command line with the HDA and the NIC paired, where the lane
    swaps (lanes.py) ask for it; unchanged otherwise."""
    kind = swaps.get("audio")
    if kind is None:
        return list(cmd)
    if kind not in KINDS:
        raise ValueError(f"no HDA pairing {kind!r}: the kinds are {', '.join(KINDS)}")
    out, moved = [], []
    for i, word in enumerate(cmd):
        device = i > 0 and cmd[i - 1] == "-device"
        if device and PORT not in out:
            out[-1:] = ["-device", PORT, "-device"]
        if device and word == "intel-hda":
            out.append(word + HDA)
            moved.append("hda")
        elif device and word.endswith(",netdev=net0"):
            out.append(word + NIC)
            moved.append("nic")
        else:
            out.append(word)
    if sorted(moved) != ["hda", "nic"]:
        raise ValueError("hda-beside-nic needs the machine's one HDA and one NIC on net0")
    return out
