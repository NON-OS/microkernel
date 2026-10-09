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

"""The disks a boot runs from, laid out as mk/10-qemu.mk always gave them:
the ESP the seal wrote, as its own FAT drive, and a virtio data disk that
carries the store and the disk plan. The kernel takes the first disk with a
store or a plan on it, which is the data disk, and opens the encrypted
volume from its plan.

The data disk is kept between boots, so the volume keeps what it holds: a
sealed model, the wallet's vault, the shield's store. A new seal, or
--fresh, starts it again from the image. A model is laid on a data disk only
when it starts again, because laying one writes a new plan for the volume.

The sealed stick itself has its ESP from 128 MiB, past where the plan goes,
so it boots with no data volume, as a stick does; --stick boots it that way."""

import os
import shutil
import struct
import subprocess
import sys

from nonos_data_plan.image import PLAN_LBA, SECTOR, clear_gpt
from nonos_seal.media import ESP_MB

MODELS = "target/models/files"
# The stick's store region: the kernel reads it from LBA 256, the disk plan
# sits at 120 MiB, and the ESP starts where the seal put it. Both are taken
# from the seal's own number: this file once kept its own 64 MiB, and when the
# store grew and the ESP moved to 128 MiB it read no ESP at all.
STORE_BYTES = ESP_MB << 20
ESP_OFFSET = f"{ESP_MB}M"


def stamp_of(data):
    return data + ".from"


def needs_copy(source, data, fresh):
    """Whether the data disk must start again from `source`."""
    if fresh or not os.path.exists(data):
        return True
    try:
        with open(stamp_of(data)) as f:
            was = f.read().split("\n")
    except OSError:
        return True
    return was[:2] != [os.path.abspath(source), str(os.stat(source).st_mtime_ns)]


def esp(source, into):
    """The ESP's files, copied out of the stick into a folder QEMU serves as
    a FAT drive. Taken again on every boot, so it is always this seal's."""
    shutil.rmtree(into, ignore_errors=True)
    os.makedirs(into)
    rc = subprocess.call(["mcopy", "-i", f"{source}@@{ESP_OFFSET}", "-s", "-n", "::/*", into])
    if rc != 0:
        sys.exit(f"nonos-qemu: could not read the ESP of {source} (mcopy {rc})")


def drop_gpt(data):
    """Take the stick's partition table off the data disk. It comes with the
    store region and says the disk ends at the stick's size, so it puts its
    backup inside the data volume, and the firmware writes that backup back
    over the volume at every boot. The data disk is found by its store and
    plan, never by a partition. Whether one was there."""
    return clear_gpt(data, 0, 1 << 62)


def store(source, data):
    """A new data disk: the stick's store region, and the files the stick's
    live plan names (the Qwen tier it carries), each at its own LBA, so the
    plan copied with the store names only what this disk holds."""
    with open(source, "rb") as src, open(data, "wb") as dst:
        dst.write(src.read(STORE_BYTES))
        for lba, size in live_imports(src):
            src.seek(lba * SECTOR)
            dst.seek(lba * SECTOR)
            left = size
            while left:
                chunk = src.read(min(left, 1 << 20))
                if not chunk:
                    sys.exit(f"nonos-qemu: {source} ends inside the file its plan names at LBA {lba}")
                dst.write(chunk)
                left -= len(chunk)
    drop_gpt(data)
    with open(stamp_of(data), "w") as f:
        f.write(f"{os.path.abspath(source)}\n{os.stat(source).st_mtime_ns}\n")


MIB = 1 << 20


def live_imports(src):
    """The (LBA, bytes) of each file a live plan on the stick names; none
    when the stick has no plan, or one with a volume."""
    src.seek(PLAN_LBA * SECTOR)
    sector = src.read(SECTOR)
    if len(sector) < 32 or sector[:8] != b"NONOSDP1":
        return []
    base, sectors, count = struct.unpack_from("<3Q", sector, 8)
    if base or sectors:
        return []
    return [struct.unpack_from("<2Q", sector, 32 + i * 16) for i in range(min(count, 30))]


def memory_bytes(mem):
    """QEMU's -m value in bytes: 8G, 8192M, or MiB with no suffix."""
    units = {"G": 1 << 30, "M": MIB, "K": 1 << 10}
    text = mem.strip().upper()
    return int(text[:-1]) * units[text[-1]] if text[-1] in units else int(text) * MIB


def fits(file, memory):
    """The setup wizard's rule (capsule_setup_wizard/src/qwen/fit.rs): the
    file and a fifth again, 512 MiB for the runtime, 1 GiB kept for the
    system."""
    return file + file // 5 + 512 * MIB <= memory - 1024 * MIB


def auto(memory):
    """The tier setup would start on for this much memory: the largest Qwen3
    tier that fits, smallest first being how setup lists them, or none."""
    sys.path.insert(0, "tools")
    from nonos_qwen_tier.pins import pins
    sizes = sorted((sum(s for _, s, _ in files), tier) for tier, files in pins().items())
    fitting = [tier for size, tier in sizes if fits(size, memory) and tier.startswith("qwen3-")]
    return fitting[-1:] if fitting else []


def resolve(tiers, mem):
    """`auto` becomes setup's own pick for the machine's memory."""
    if "auto" not in tiers:
        return tiers
    picked = auto(memory_bytes(mem))
    print(f"nonos-qemu: --model auto, for {mem} of memory: {' '.join(picked) or 'no tier fits'}", flush=True)
    return [t for t in tiers if t != "auto"] + [t for t in picked if t not in tiers]


def lay_models(data, tiers):
    """Fetch each tier's pinned files once into target/models/files, checked
    against their pins, and lay them on the data disk as imports for their
    first use, as nonos-qwen-tier.py plan does for any disk."""
    os.makedirs(MODELS, exist_ok=True)
    tool = [sys.executable, "tools/nonos-qwen-tier.py"]
    for verb, extra in (("fetch", []), ("plan", ["--image", data, "--fresh"])):
        rc = subprocess.call(tool + [verb, *tiers, "--dir", MODELS, *extra])
        if rc != 0:
            sys.exit(f"nonos-qemu: nonos-qwen-tier.py {verb} {' '.join(tiers)} failed ({rc})")


def prepare(source, esp_dir, data, fresh, tiers):
    esp(source, esp_dir)
    if needs_copy(source, data, fresh):
        print(f"nonos-qemu: a new data disk from {source}", flush=True)
        store(source, data)
        if tiers:
            lay_models(data, tiers)
        return
    print("nonos-qemu: the data disk of the last boot, its volume kept (--fresh starts again)", flush=True)
    if drop_gpt(data):
        print("nonos-qemu: removed the stick's partition table from the data disk; firmware had "
              "been writing its backup into the volume at every boot, so a model sealed there "
              "may fail to read: --fresh lays it again", flush=True)
    if tiers:
        print("nonos-qemu: models are laid on a new data disk only; add --fresh to lay "
              f"{' '.join(tiers)} (the volume starts empty)", flush=True)
