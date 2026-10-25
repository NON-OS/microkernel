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


"""The three boots, as commands, each writing one serial log. No boot reads
or makes a key in this checkout: the image and the make lanes are built in
the copy tools/nonos-dev-image keeps under target/dev/tree, with the
throwaway keys it makes there (guard.py holds every boot to that)."""

import collections
import os

TREE = "target/dev/tree"
DEV_IMAGE = f"{TREE}/target/release/selftest-dev/nonos.img"

# log: its serial log's absolute path. until: patterns that, once all are in
# the log, end the boot after `settle` seconds; with none the command ends by
# itself. cwd: where it runs. env: what it adds to the environment.
Boot = collections.namedtuple("Boot", "name cmd log until settle cwd env")


def in_copy(root):
    """A make lane's place and its signing key: the copy's, never ours."""
    tree = os.path.join(root, TREE)
    return tree, {"NONOS_SIGNING_KEY": os.path.join(tree, ".keys/signing_key_v1.bin")}


def capsule(root, out, iommu):
    """The selftest image, the attack capsule last at boot. It runs on after
    the capsule's fault, to show the kernel went on. One CPU: the kernel's
    trap dump is several writes, which another CPU's line could split."""
    log = os.path.join(out, "capsule.log")
    cmd = ["nix", "run", ".#qemu", "--", "--image", os.path.join(root, DEV_IMAGE), "--fresh",
           "--headless", "--smp", "1", "--serial", log, *(["--lane", "iommu=intel"] if iommu else [])]
    return Boot("capsule", cmd, log, [r"\[ATTACK-FAULT\]"], 30, root, {})


def attest(root, out):
    """The spawn gate against tampered trailers, make's lane in the copy; it
    stops its QEMU at 240 s."""
    log = os.path.join(out, "attest.log")
    cmd = ["make", "--no-print-directory", "nonos-mk-attest-refusal-run", f"ATTEST_REFUSAL_LOG={log}"]
    return Boot("attest", cmd, log, [], 0, *in_copy(root))


def linux(root, out):
    """/bin/suite under the Linux personality, make's lane in the copy."""
    log = os.path.join(out, "linux.log")
    cmd = ["make", "--no-print-directory", "nonos-mk-run-serial-log", "NONOS_DEV=1",
           "NONOS_LINUX_GUESTS=1", "LINUX_GUEST_SET=suite", f"QEMU_SERIAL_LOG={log}"]
    until = [rf"\[ATTACK\] {a} " for a in ("linux-fs-escape", "linux-foreign-memory", "linux-raw-syscall")]
    return Boot("linux", cmd, log, until, 10, *in_copy(root))


NAMES = ("capsule", "attest", "linux")


def plan(only, root, out, iommu=False):
    out = os.path.abspath(os.path.join(root, out))
    made = {"capsule": lambda: capsule(root, out, iommu), "attest": lambda: attest(root, out),
            "linux": lambda: linux(root, out)}
    return [made[n]() for n in (only or NAMES)]
