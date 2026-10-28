# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""The MediaTek firmware package's pins, read from the one place they are
written: the drivers' own tables, the Wi-Fi driver's six files
(userland/capsule_driver_mt7921/src/fw/pins.rs) then the Bluetooth
capsule's two (userland/capsule_driver_bt_usb/src/vendor/mediatek/pins.rs).
Shared by scripts/firmware_package.py, which places the files on the boot
medium, and scripts/check_mediatek_firmware.py, which holds the loader, the
kernel and the tree to them."""

import re
from pathlib import Path

PINS_RS = "userland/capsule_driver_mt7921/src/fw/pins.rs"
PACKAGE_RS = "userland/capsule_driver_mt7921/src/fw/package.rs"
BT_PINS_RS = "userland/capsule_driver_bt_usb/src/vendor/mediatek/pins.rs"
BT_FILES_RS = "userland/capsule_driver_bt_usb/src/vendor/mediatek/files.rs"
# The Wi-Fi driver's files, then the Bluetooth capsule's, in slot order.
COUNTS = ((PINS_RS, 6), (BT_PINS_RS, 2))
LOADER_RS = "nonos-bootloader/src/entry/firmware_package.rs"
KINDS_RS = "src/syscall/microkernel/install_source_kinds.rs"
# linux-firmware's licence for these files, by its sha256 at the commit.
LICENCE = ("LICENSES/LICENCE.mediatek",
           "a90d3f66704d85889945fec5525ea77622549da83aced1aac99828383f8f1805")
UPSTREAM = "https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git/plain/"
# Where on the boot medium, under the ESP's EFI/nonos.
ESP_DIR = "firmware/mediatek"


def commit(root: Path) -> str:
    revs = set()
    for rel, _ in COUNTS:
        text = (root / rel).read_text()
        found = re.search(r"commit\s*//!?\s*([0-9a-f]{40})|commit\s+([0-9a-f]{40})", text)
        if not found:
            raise SystemExit(f"{rel}: no linux-firmware commit named")
        revs.add(found.group(1) or found.group(2))
    if len(revs) != 1:
        raise SystemExit(f"{PINS_RS} and {BT_PINS_RS} name different commits")
    return revs.pop()


def pins(root: Path):
    """[(name, length, sha256)] in slot order."""
    out = []
    for rel, want in COUNTS:
        text = (root / rel).read_text()
        rows = re.findall(r'\(\s*"([^"]+)",\s*(\d+),\s*"([0-9a-f]{64})",\s*\)', text)
        if len(rows) != want:
            raise SystemExit(f"{rel}: {len(rows)} pins read, {want} expected")
        out += [(n, int(l), h) for n, l, h in rows]
    return out
