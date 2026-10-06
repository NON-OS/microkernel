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

"""Booting a sealed NONOS image under QEMU, the way the hardware would: the
USB image the seal wrote (the store and the ESP the installer copies from) on
a q35 machine with UEFI firmware, a software TPM when asked, and an NVMe disk
to install to. Every tool comes from the flake, so a boot is the same on
Linux, macOS and WSL2."""
