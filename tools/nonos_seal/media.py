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

"""Phase 5, the media: the EFI system partition, the package store, the USB
image and the ISO. The ESP holds exactly the files the installer writes to a
disk (userland/nonos_disk/src/image/nonos.rs), with the same boot.cfg and
startup.nsh, so the live stick and the installed disk boot the same files."""

import json
import os
import shutil
import subprocess

ESP_FILES = "EFI/nonos"
BOOT_CFG = b"timeout=0\ndefault=nonos\n"
STARTUP_NSH = b"\\EFI\\BOOT\\BOOTX64.EFI\r\n"

# The stick is laid out as an installed disk is: the store at LBA 256, below
# the disk plan at 120 MiB, and the ESP from 128 MiB
# (userland/nonos_disk_map/src/places.rs).
STORE_LBA = "256"
USB_MB, ESP_MB = 384, 128
# The Qwen tier the stick carries, so Qwen answers offline on any machine.
# It is laid past the ESP under a live plan (src/fs/blockfs_volume/
# plan_types.rs): the stick keeps no volume, and the kernel imports the
# tier into the session's volume in RAM only when its digest is the one the
# signed catalogue pins. Every other tier is chosen and fetched.
STICK_TIER = "qwen3-0.6b"
MODELS = "target/models/files"
# Kept empty after the last import, where the partition table's backup goes.
GPT_TAIL = 2048
# Every timestamp that reaches an image is pinned, so the media's own bytes
# carry no clock.
IMAGE_DATE, IMAGE_TOUCH, FAT_SERIAL = "2026010100000000", "202601010000.00", "4e4f4e4f"


def esp(out, loader, kernel, loader_trailer, boot_root, approval):
    root = os.path.join(out, "esp")
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(os.path.join(root, "EFI/BOOT"))
    os.makedirs(os.path.join(root, ESP_FILES))
    place = {"EFI/BOOT/BOOTX64.EFI": loader, f"{ESP_FILES}/kernel.bin": kernel,
             f"{ESP_FILES}/bootloader.trailer": loader_trailer, f"{ESP_FILES}/boot_root.approval": boot_root}
    if approval:
        place[f"{ESP_FILES}/kernel.approval"] = approval
    for rel, src in place.items():
        shutil.copyfile(src, os.path.join(root, rel))
    with open(os.path.join(root, ESP_FILES, "boot.cfg"), "wb") as f:
        f.write(BOOT_CFG)
    with open(os.path.join(root, "startup.nsh"), "wb") as f:
        f.write(STARTUP_NSH)
    return root


def store_entries(store, capsules_out, linux_out, catalogue, enabled, out):
    """The store make declares (tools/nix/store.json), each file found where
    the flake built it: a capsule ELF in the capsules output, a Linux program
    in the userland output, the wallpaper collection packed into `out`,
    anything else in the tree. The collection goes in only when the image has
    the wallpaper catalog that reads it."""
    by_bin = {f"{e['dir']}/target/{e['target']}/release/{e['bin']}": os.path.join(capsules_out, e["slug"], e["bin"])
              for e in catalogue}
    with open("tools/nix/store.json") as f:
        groups = json.load(f)
    entries = []
    wanted = [g for g in ("demo", "media", "linux") if store.get(g, True)]
    if "nonos-capsule-wallpaper-catalog" in enabled:
        wanted.append("wallpapers")
    for group in wanted:
        for entry in groups[group]:
            src = entry["file"]
            if src in by_bin:
                src = by_bin[src]
            elif src.startswith("target/linux-userland/"):
                src = os.path.join(linux_out, os.path.relpath(src, "target/linux-userland"))
            elif src == "target/wallpapers/collection":
                src = os.path.join(out, "wallpapers", "collection")
                subprocess.run(["python3", "tools/nonos-wallpaper-pack", "pack", src], check=True)
            entries += ["--entry", f"{entry['path']}={src}"]
    return entries


def usb(step, out, esp_root, entries):
    img = os.path.join(out, "nonos.img")
    with open(img, "wb") as f:
        f.truncate(USB_MB << 20)
    step.run("sgdisk", "-o", "-n", f"1:{ESP_MB * 2048}:0", "-t", "1:EF00", "-c", "1:NONOS-ESP", img)
    part = f"{img}@@{ESP_MB}M"
    step.run("mformat", "-i", part, "-N", FAT_SERIAL, "-F", "::")
    step.run("mcopy", "-i", part, "-s", "-m", os.path.join(esp_root, "EFI"), "::/EFI")
    step.run("mcopy", "-i", part, "-m", os.path.join(esp_root, "startup.nsh"), "::/")
    step.run("python3", "tools/nonos-store-pack", "--image", img, "--lba", STORE_LBA, *entries)
    stick_tier(step, img)
    return img


def stick_tier(step, img):
    """Lays STICK_TIER's pinned files past the ESP, under a live plan, and
    moves the partition table's backup to the stick's new end."""
    import sys
    sys.path.insert(0, "tools")
    from nonos_qwen_tier.pins import pins
    from nonos_qwen_tier.upstream import upstream
    step.run("python3", "tools/nonos-qwen-tier.py", "fetch", STICK_TIER, "--dir", MODELS)
    files = [os.path.join(MODELS, upstream(name)) for name, _, _ in pins()[STICK_TIER]]
    step.run("python3", "tools/nonos-data-plan.py", img, "--live-at", str(USB_MB * 2048),
             *[x for f in files for x in ("--import", f)])
    with open(img, "r+b") as f:
        f.truncate(f.seek(0, os.SEEK_END) + GPT_TAIL * 512)
    step.run("sgdisk", "-e", img)


def iso(step, out, esp_root):
    work = os.path.join(out, "isoroot")
    shutil.rmtree(work, ignore_errors=True)
    shutil.copytree(os.path.join(esp_root, "EFI"), os.path.join(work, "EFI"))
    size = sum(os.path.getsize(os.path.join(d, n)) for d, _, ns in os.walk(work) for n in ns)
    boot = os.path.join(work, "efiboot.img")
    with open(boot, "wb") as f:
        f.truncate(((size >> 20) + 16) << 20)
    step.run("find", work, "-exec", "touch", "-t", IMAGE_TOUCH, "{}", "+")
    step.run("mformat", "-i", boot, "-N", FAT_SERIAL, "-F", "::")
    step.run("mcopy", "-i", boot, "-s", "-m", os.path.join(work, "EFI"), "::/EFI")
    step.run("touch", "-t", IMAGE_TOUCH, boot)
    target = os.path.join(out, "nonos.iso")
    step.run("xorriso", "-as", "mkisofs", "-R", "-J", "-V", "NONOS", "-e", "efiboot.img", "-no-emul-boot",
             f"--modification-date={IMAGE_DATE}", "-o", target, work)
    shutil.rmtree(work)
    return target
