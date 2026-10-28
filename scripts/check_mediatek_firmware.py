#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
"""Hold the MediaTek firmware package to its one set of pins, and keep its
bytes out of the tree.

The mt7921e driver pins each of its six files by name, length and sha256
(userland/capsule_driver_mt7921/src/fw/pins.rs), and the Bluetooth capsule
its two after them (userland/capsule_driver_bt_usb/src/vendor/mediatek/pins.rs).
Fails when:
  - the nix artifact's list (tools/nix/firmware.json) names another commit,
    file, length or hash, or another licence;
  - the loader's paths (nonos-bootloader/src/entry/firmware_package.rs) name
    other files, in another order, or another first module kind;
  - the kernel's served kinds (install_source_kinds.rs) are not 0x20 and
    eight, the Wi-Fi driver's first slot not 0x20 or the Bluetooth
    capsule's first kind not 0x26;
  - the loader reads a file without first asking whether it is there (a
    medium without the package must boot without a "File not found" line);
  - the seal's ESP (tools/nonos_seal/media.py, run here on empty stand-ins)
    does not place the artifact tree's files where the loader reads them, or
    the seal does not hand it the artifact tree's firmware/mediatek;
  - any tracked file is one of the pinned files (by length and sha256), a
    file under nonos-bootloader/firmware/mediatek/ other than its licence
    is not empty, or a Rust source includes a MediaTek file's bytes;
  - with --blobs DIR (a directory holding the pinned files), any tracked
    source or any --artifact (a built capsule or kernel) holds a run of a
    pinned file's bytes, raw or as a Rust byte literal.

    check_mediatek_firmware.py [--blobs DIR] [--artifact FILE ...] [--self-test]
"""

import argparse
import hashlib
import importlib.util
import json
import random
import re
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from mediatek_firmware import (BT_FILES_RS, BT_PINS_RS, ESP_DIR, KINDS_RS, LICENCE,  # noqa: E402
                               LOADER_RS, PACKAGE_RS, commit, pins)

NIX_JSON = "tools/nix/firmware.json"
SEAL_MEDIA = "tools/nonos_seal/media.py"
SEAL_MAIN = "tools/nonos_seal/__main__.py"
PLACEHOLDERS = "nonos-bootloader/firmware/mediatek/"
SCOPE = ("src/", "userland/", "nonos-bootloader/", "kernel_proofs/")
NEEDLE = 32
STRIDE = 8192
DISTINCT = 24
INCLUDE = re.compile(r"include_bytes!\s*\([^)]*(mediatek|WIFI_RAM_CODE|WIFI_MT79|BT_RAM_CODE)", re.I)
HEXRUN = re.compile(r"(?:0x[0-9a-fA-F]{1,2}\s*,\s*){32,}")


def number(text, name):
    found = re.search(rf"{name}\s*:\s*u\d+\s*=\s*(0x[0-9a-fA-F]+|\d+)", text)
    return int(found.group(1), 0) if found else None


def agreement(root: Path):
    out, rev, pinned = [], commit(root), pins(root)
    nix = json.loads((root / NIX_JSON).read_text())
    want = [{"path": f"mediatek/{n}", "size": l, "sha256": h} for n, l, h in pinned]
    want.append({"path": LICENCE[0], "sha256": LICENCE[1]})
    if nix.get("commit") != rev or nix.get("files") != want:
        out.append(f"{NIX_JSON}: not the driver's pins at {rev[:12]}")
    loader = (root / LOADER_RS).read_text()
    paths = re.findall(r'cstr16!\("\\\\EFI\\\\nonos\\\\firmware\\\\mediatek\\\\([^"]+)"\)', loader)
    if paths != [n for n, _, _ in pinned]:
        out.append(f"{LOADER_RS}: reads {paths}, not the pinned files in slot order")
    if number(loader, "KIND_FIRMWARE_FIRST") != 0x20:
        out.append(f"{LOADER_RS}: KIND_FIRMWARE_FIRST is not 0x20")
    kinds = (root / KINDS_RS).read_text()
    if number(kinds, "KIND_FIRMWARE_FIRST") != 0x20 or number(kinds, "FIRMWARE_FILES") != len(pinned):
        out.append(f"{KINDS_RS}: does not serve kinds 0x20..0x{0x1f + len(pinned):02x}")
    if number((root / PACKAGE_RS).read_text(), "SLOT_FIRST") != 0x20:
        out.append(f"{PACKAGE_RS}: SLOT_FIRST is not 0x20")
    if number((root / BT_FILES_RS).read_text(), "KIND_FIRST") != 0x26:
        out.append(f"{BT_FILES_RS}: KIND_FIRST is not 0x26, the slot after the Wi-Fi six")
    asked, read = loader.find("if !file_exists(st, path)"), loader.find("load_file_from_esp(st, path)")
    if asked < 0 or read < asked:
        out.append(f"{LOADER_RS}: reads a file without asking first whether it is there")
    return out + placed(root, [n for n, _, _ in pinned])


def placed(root: Path, names):
    """The seal's ESP, built from empty stand-ins: every pinned file and the
    licence at EFI/nonos/<ESP_DIR>/, which is where the loader reads them."""
    if "firmware\\\\mediatek" not in (root / LOADER_RS).read_text() or ESP_DIR != "firmware/mediatek":
        return [f"{LOADER_RS}: not the package's place, EFI/nonos/{ESP_DIR}"]
    if not re.search(r'media\.esp\([^)]*\bfirmware\)', (root / SEAL_MAIN).read_text()) or \
            '"firmware", "mediatek"' not in (root / SEAL_MAIN).read_text():
        return [f"{SEAL_MAIN}: does not hand the ESP the artifact tree's firmware/mediatek"]
    spec = importlib.util.spec_from_file_location("seal_media", root / SEAL_MEDIA)
    media = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(media)
    with tempfile.TemporaryDirectory() as d:
        fw, out = Path(d) / "fw", Path(d) / "out"
        fw.mkdir()
        for n in names + ["LICENCE.mediatek"]:
            (fw / n).write_bytes(b"")
        stand = Path(d) / "stand-in"
        stand.write_bytes(b"")
        esp = Path(media.esp(str(out), *[str(stand)] * 5, str(fw)))
        missing = [n for n in names + ["LICENCE.mediatek"]
                   if not (esp / "EFI" / "nonos" / ESP_DIR / n).is_file()]
    return [f"{SEAL_MEDIA}: the seal's ESP lacks {missing}"] if missing else []


def tracked(root: Path):
    try:
        names = subprocess.run(["git", "ls-files", "-z"], cwd=root, capture_output=True,
                               check=True).stdout.decode().split("\0")
    except (OSError, subprocess.CalledProcessError):
        names = [str(p.relative_to(root)) for p in root.rglob("*") if p.is_file()]
    return [n for n in names if n and (root / n).is_file()]


def needles(blobs: Path, pinned):
    found = set()
    for name, _, _ in pinned:
        data = (blobs / name).read_bytes()
        found |= {data[i:i + NEEDLE] for i in range(0, len(data) - NEEDLE, STRIDE)}
    # Padding and tables (runs of one byte, short cycles) are in any file;
    # only a needle of many distinct bytes says it came from these.
    return {c for c in found if len(set(c)) >= DISTINCT}


def holds(data: bytes, chunks) -> bool:
    return any(c in data for c in chunks)


def literal_bytes(text: str) -> bytes:
    return b"".join(bytes(int(x, 16) for x in re.findall(r"0x([0-9a-fA-F]{1,2})", run.group(0)))
                    for run in HEXRUN.finditer(text))


def no_blobs(root: Path, blobs=None, artifacts=()):
    out, pinned = [], pins(root)
    by_len = {l: h for _, l, h in pinned}
    chunks = needles(blobs, pinned) if blobs else set()
    for name in tracked(root):
        path = root / name
        size = path.stat().st_size
        if size in by_len and hashlib.sha256(path.read_bytes()).hexdigest() == by_len[size]:
            out.append(f"{name}: is a pinned MediaTek file")
        if name.startswith(PLACEHOLDERS) and size and Path(name).name != "LICENSE":
            out.append(f"{name}: MediaTek firmware bytes in the tree")
        if name.endswith(".rs") and INCLUDE.search(path.read_text(errors="replace")):
            out.append(f"{name}: includes a MediaTek firmware file")
        if chunks and name.startswith(SCOPE):
            data = path.read_bytes()
            lit = literal_bytes(data.decode(errors="replace")) if name.endswith(".rs") else b""
            if holds(data, chunks) or holds(lit, chunks):
                out.append(f"{name}: holds bytes of a pinned MediaTek file")
    for art in artifacts:
        if holds(Path(art).read_bytes(), chunks):
            out.append(f"{art}: holds bytes of a pinned MediaTek file")
    return out


def self_test() -> int:
    root = Path(__file__).resolve().parent.parent
    if agreement(root):
        print("self-test: the tree itself disagrees", agreement(root))
        return 1
    with tempfile.TemporaryDirectory() as d, tempfile.TemporaryDirectory() as b:
        t, blobs = Path(d), Path(b)
        for rel in ("userland/capsule_driver_mt7921/src/fw/pins.rs", PACKAGE_RS, LOADER_RS,
                    KINDS_RS, NIX_JSON, SEAL_MEDIA, SEAL_MAIN, "scripts/mediatek_firmware.py",
                    BT_PINS_RS, BT_FILES_RS):
            (t / rel).parent.mkdir(parents=True, exist_ok=True)
            (t / rel).write_text((root / rel).read_text())
        if agreement(t) or no_blobs(t):
            print("self-test: a faithful copy fails")
            return 1
        name, length, _ = pins(t)[0]
        blob = bytes((i * 7 + 3) & 0xff for i in range(length))
        pins_rs = t / "userland/capsule_driver_mt7921/src/fw/pins.rs"
        text = pins_rs.read_text()
        pins_rs.write_text(text.replace(pins(t)[0][2], hashlib.sha256(blob).hexdigest()))
        for i, (n, l, _) in enumerate(pins(t)):
            (blobs / n).write_bytes(blob if n == name else random.Random(i).randbytes(l))
        if no_blobs(t, blobs):
            print("self-test: a clean tree is found holding the files")
            return 1
        cases = []
        (t / "userland/x.bin").write_bytes(blob)
        cases.append(("a pinned file committed", no_blobs(t)))
        (t / "userland/x.bin").unlink()
        lit = ", ".join(f"0x{b:02x}" for b in blob[STRIDE:STRIDE + 64])
        (t / "userland/y.rs").write_text(f"const B: [u8; 64] = [{lit}];\n")
        cases.append(("its bytes as a Rust literal", no_blobs(t, blobs)))
        (t / "userland/y.rs").write_text('static F: &[u8] = include_bytes!("mediatek/a.bin");\n')
        cases.append(("an include_bytes! of it", no_blobs(t)))
        (t / "userland/y.rs").unlink()
        (t / PLACEHOLDERS).mkdir(parents=True)
        (t / PLACEHOLDERS / "mt7922_fw.bin").write_bytes(b"\x01")
        cases.append(("a placeholder filled", no_blobs(t)))
        (t / PLACEHOLDERS / "mt7922_fw.bin").write_bytes(b"")
        art = t / "capsule.elf"
        art.write_bytes(b"\0" * 100 + blob[:STRIDE + NEEDLE] + b"\0")
        cases.append(("a built capsule holding it", no_blobs(t, blobs, [art])))
        pins_rs.write_text(text)
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text().replace("MT7922_1.bin", "MT7922_2.bin"))
        cases.append(("the loader reading another file", agreement(t)))
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text())
        (t / KINDS_RS).write_text((root / KINDS_RS).read_text().replace("FIRMWARE_FILES: u32 = 8", "FIRMWARE_FILES: u32 = 6"))
        cases.append(("the kernel serving the Wi-Fi six alone", agreement(t)))
        (t / KINDS_RS).write_text((root / KINDS_RS).read_text())
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text().replace(
            "BT_RAM_CODE_MT7922_1_1_hdr.bin", "BT_RAM_CODE_MT7922_1_2_hdr.bin"))
        cases.append(("the loader reading another Bluetooth file", agreement(t)))
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text())
        (t / BT_FILES_RS).write_text((root / BT_FILES_RS).read_text().replace(
            "KIND_FIRST: u64 = 0x26", "KIND_FIRST: u64 = 0x20"))
        cases.append(("the Bluetooth capsule asking for the Wi-Fi kinds", agreement(t)))
        (t / BT_FILES_RS).write_text((root / BT_FILES_RS).read_text())
        (t / NIX_JSON).write_text((root / NIX_JSON).read_text().replace("533870", "533871"))
        cases.append(("the nix list another Bluetooth length", agreement(t)))
        (t / NIX_JSON).write_text((root / NIX_JSON).read_text())
        (t / "userland/z.rs").write_text('static B: &[u8] = include_bytes!("BT_RAM_CODE_MT7922_1_1_hdr.bin");\n')
        cases.append(("an include_bytes! of a Bluetooth file", no_blobs(t)))
        (t / "userland/z.rs").unlink()
        (t / KINDS_RS).write_text((root / KINDS_RS).read_text())
        (t / NIX_JSON).write_text((root / NIX_JSON).read_text().replace("92192", "92193"))
        cases.append(("the nix list another length", agreement(t)))
        (t / NIX_JSON).write_text((root / NIX_JSON).read_text())
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text().replace("if !file_exists(st, path)", "if false"))
        cases.append(("the loader reading without asking", agreement(t)))
        (t / LOADER_RS).write_text((root / LOADER_RS).read_text())
        (t / SEAL_MEDIA).write_text((root / SEAL_MEDIA).read_text().replace("if firmware and", "if False and"))
        cases.append(("the seal packing no package", agreement(t)))
        (t / SEAL_MEDIA).write_text((root / SEAL_MEDIA).read_text())
        (t / SEAL_MAIN).write_text((root / SEAL_MAIN).read_text().replace(", approval, firmware)", ", approval)"))
        cases.append(("the seal handing the ESP nothing", agreement(t)))
    missed = [what for what, found in cases if not found]
    print(f"self-test: {len(cases) - len(missed)} of {len(cases)} caught; missed: {missed}")
    return 1 if missed else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--blobs", type=Path)
    ap.add_argument("--artifact", action="append", default=[])
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    if a.artifact and not a.blobs:
        ap.error("--artifact needs --blobs")
    root = Path(__file__).resolve().parent.parent
    bad = agreement(root) + no_blobs(root, a.blobs, a.artifact)
    for line in bad:
        print(f"check_mediatek_firmware: {line}")
    if not bad:
        print(f"check_mediatek_firmware: {len(pins(root))} pinned files agree in drivers, loader, kernel and "
              f"build; none in the tree{', the sources or the artifacts' if a.blobs else ''}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
