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
"""Virtio devices as an IOMMU cell must give them (plan/50 C3, K5).

Under an IOMMU the kernel confines a virtio device only when it is modern
and offers ACCESS_PLATFORM, and refuses it otherwise. So every virtio device
an IOMMU cell carries is given both, and the one cell that expects the
refusal adds a plain transitional virtio-rng instead.
"""

CONFINED = ",disable-legacy=on,iommu_platform=on"
LEGACY_RNG = ["-device", "virtio-rng-pci"]


def is_virtio(device):
    kind = device.split(",", 1)[0]
    return kind.startswith("virtio-") and (kind.endswith("-pci") or kind == "virtio-vga")


def confine(argv):
    """`argv` with every -device virtio-* given CONFINED, once."""
    out, after_device = [], False
    for arg in argv:
        if after_device and is_virtio(arg) and "iommu_platform" not in arg:
            arg += CONFINED
        out.append(arg)
        after_device = arg == "-device"
    return out


REFUSED = "refused: virtio without ACCESS_PLATFORM under an IOMMU"


def verdict(cell, text):
    """The refusal where the cell expects one, and none anywhere else."""
    if cell.legacy_virtio and REFUSED not in text:
        return ["the transitional virtio-rng was not refused under the IOMMU"]
    if not cell.legacy_virtio and REFUSED in text:
        return ["a virtio device was refused in a cell that gives each ACCESS_PLATFORM"]
    return []


def self_test():
    argv = ["-device", "virtio-blk-pci,drive=vd0", "-device", "qemu-xhci,id=xhci",
            "-device", "virtio-vga", "-drive", "virtio-x-pci", "-device", "virtio-rng-pci" + CONFINED]
    got = confine(argv)
    assert got[1] == "virtio-blk-pci,drive=vd0" + CONFINED, got
    assert got[3] == "qemu-xhci,id=xhci" and got[5] == "virtio-vga" + CONFINED, got
    assert got[7] == "virtio-x-pci", "a -drive value is not a device"
    assert got[9].count("iommu_platform") == 1, "given once"
    print("bootmatrix virtio: self-test PASS")


if __name__ == "__main__":
    self_test()
