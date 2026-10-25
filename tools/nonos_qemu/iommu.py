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

"""The IOMMU lanes: iommu=intel (VT-d) and iommu=amd (AMD-Vi), each with
interrupt remapping on. A lane here changes the whole machine rather than
adding one device, so it works on the finished command line:

- interrupt remapping needs the interrupt controller split between QEMU and
  the guest, so the machine gets kernel-irqchip=split;
- the IOMMU goes in straight after the machine, before any PCI device;
- every virtio device is made modern-only and told to send its DMA through
  the IOMMU (ACCESS_PLATFORM). The kernel refuses a virtio device without it
  under an IOMMU, so leaving one out would read as a driver failure."""

UNITS = {
    "intel": "intel-iommu,intremap=on,caching-mode=on",
    "amd": "amd-iommu,intremap=on,dma-remap=on",
}

THROUGH = ",disable-legacy=on,iommu_platform=on"


def behind(device):
    """A virtio PCI device's value, made to send its DMA through the IOMMU."""
    model, sep, rest = device.partition(",")
    if not model.startswith("virtio-") or not (model.endswith("-pci") or model == "virtio-vga"):
        return device
    return model + THROUGH + sep + rest


def apply(cmd, kind):
    """The command line with the IOMMU of `kind` in it; unchanged for None."""
    if kind is None:
        return list(cmd)
    if kind not in UNITS:
        raise ValueError(f"no IOMMU {kind!r}: the kinds are {', '.join(UNITS)}")
    out, i = [], 0
    while i < len(cmd):
        word = cmd[i]
        if word == "-machine" and i + 1 < len(cmd):
            out += [word, cmd[i + 1] + ",kernel-irqchip=split", "-device", UNITS[kind]]
            i += 2
            continue
        if word == "-device" and i + 1 < len(cmd):
            out += [word, behind(cmd[i + 1])]
            i += 2
            continue
        out.append(word)
        i += 1
    if "-machine" not in cmd:
        raise ValueError("an IOMMU lane needs the -machine the IOMMU sits on")
    return out
