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
"""nonos_system_map's paths, capability names and colours, and its pace,
which main may set to zero."""

import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.basename(ROOT)
TRUST = os.path.join(ROOT, "nonos-data", "trust")
USERLAND = os.path.join(ROOT, "userland")
TARGET = os.environ.get("NONOS_USER_TARGET", "x86_64-nonos-user")

EM_MACHINES = {62: "x86-64", 183: "AArch64", 243: "RISC-V"}

# Mirrors the capability list in src/capabilities/types/defs.rs, the one place the
# kernel declares them. A bit the kernel knows and this does not is reported as its
# number rather than dropped, so the two cannot drift apart quietly. ForeignExec was
# missing here and from the kernel's own array alike: a list kept by hand beside another.
CAPABILITIES = [
    (1 << 0, "CoreExec"), (1 << 1, "IO"), (1 << 2, "Network"), (1 << 3, "IPC"),
    (1 << 4, "Memory"), (1 << 5, "Crypto"), (1 << 6, "FileSystem"),
    (1 << 7, "Hardware"), (1 << 8, "Debug"), (1 << 9, "Admin"),
    (1 << 10, "RegisterService"), (1 << 11, "GfxQuery"), (1 << 12, "GfxCreate"),
    (1 << 13, "GfxMap"), (1 << 14, "GfxPresent"), (1 << 15, "DeviceEnum"),
    (1 << 16, "Driver"), (1 << 17, "Mmio"), (1 << 18, "Irq"), (1 << 19, "Dma"),
    (1 << 20, "Pio"), (1 << 21, "InputSource"), (1 << 22, "TimeSet"),
    (1 << 23, "SpawnBroker"), (1 << 24, "SpawnWindow"), (1 << 25, "ProcessControl"),
    (1 << 26, "StoreWrite"), (1 << 27, "EnrolDevRoot"), (1 << 28, "Keyring"),
    (1 << 29, "Entropy"), (1 << 30, "AppInstall"), (1 << 31, "AttestRead"),
    (1 << 32, "ForeignExec"), (1 << 33, "LocalSign"), (1 << 34, "StreamImport"),
    (1 << 35, "DeviceSecret"), (1 << 36, "InputConsumer"), (1 << 37, "CameraCapture"),
    (1 << 38, "MicrophoneCapture"), (1 << 39, "ScreenCapture"),
]

# Authority that lets its holder act on something it does not own: put pixels
# on the screen, start or stop another process, mint a service name, move the
# clock. A second holder appearing against any of these is a change that wants
# explaining, which is the whole reason to print them separately.
SCARCE = {
    "Admin", "RegisterService", "GfxPresent", "TimeSet",
    "SpawnBroker", "SpawnWindow", "ProcessControl", "Pio", "ForeignExec",
    "LocalSign", "StreamImport", "DeviceSecret", "InputConsumer", "CameraCapture",
    "MicrophoneCapture", "ScreenCapture",
}

BOLD = "\033[1m"
DIM = "\033[2m"
CYAN = "\033[36m"
GREEN = "\033[32m"
YELLOW = "\033[33m"
RED = "\033[31m"
OFF = "\033[0m"

DELAY = 0.006
PAUSE = 0.35
