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
"""A USB mass-storage stick on the xHCI controller, for a cell's store."""


def usb_stick(argv, path, block):
    """The stick on the xHCI controller the extra devices carry, or on one of
    its own when they carry none."""
    stick = "usb-storage,bus=xhci.0,drive=usbstick,removable=on"
    if block != 512:
        stick += f",logical_block_size={block},physical_block_size={block}"
    controller = [] if any(a.startswith("qemu-xhci,id=xhci") for a in argv) else ["-device", "qemu-xhci,id=xhci"]
    return controller + ["-drive", f"file={path},if=none,id=usbstick,format=raw", "-device", stick]
