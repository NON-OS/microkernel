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

"""The USB lanes' QEMU lines: a host controller with a keyboard on port 1
and a stick on port 2, and a USB speaker on the xHCI controller machine.py
always adds."""

from .lane_disks import drive

STICK_BYTES = 256 << 20


def keyboard_and_stick(bus, path, ident):
    return ["-device", f"usb-kbd,bus={bus},port=1", *drive(path, ident),
            "-device", f"usb-storage,bus={bus},port=2,drive={ident},removable=on"]


def ehci(path):
    """EHCI with its three UHCI companions, wired as on an Intel board. The
    stick runs at high speed on EHCI. The keyboard is full speed, which
    EHCI alone cannot reach, so its port is handed to a companion, as on
    hardware."""
    args = ["-device", "ich9-usb-ehci1,id=lane-ehci,addr=1d.7,multifunction=on"]
    for n in range(3):
        args += ["-device", f"ich9-usb-uhci{n + 1},id=lane-ehci-uhci{n + 1},addr=1d.{n},"
                 f"multifunction=on,masterbus=lane-ehci.0,firstport={2 * n}"]
    return args + keyboard_and_stick("lane-ehci.0", path, "lane-ehci-stick")


def uhci(path):
    return ["-device", "piix3-usb-uhci,id=lane-uhci",
            *keyboard_and_stick("lane-uhci.0", path, "lane-uhci-stick")]


def ohci(path):
    return ["-device", "pci-ohci,id=lane-ohci", *keyboard_and_stick("lane-ohci.0", path, "lane-ohci-stick")]


def usb_audio(backend):
    """A USB speaker and microphone, with an audio backend of its own: a make
    target may define no other."""
    return ["-audiodev", f"{backend},id=lane-snd", "-device", "usb-audio,audiodev=lane-snd,bus=xhci.0"]
