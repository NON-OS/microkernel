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

"""Phases 3 to 5: the kernel and the bootloader, each built by the flake from
the tree the previous phase left, enrolled under its own root; then the kernel
signed and its trailer embedded, the two release records, and Secure Boot.
The tool calls and their order are the ones nonos-mk-esp ran."""

import os
import platform
import shutil
import sys

from . import keys
from .run import nix_build, say

KERNEL_ROOT = f"{keys.TRUST}/policy/kernel_attest_root.bin"
LOADER_ROOT = f"{keys.TRUST}/policy/bootloader_attest_root.bin"
BOOT_ROOT_RECORD = f"{keys.TRUST}/policy/boot_root.approval"


def head(path, n=32):
    with open(path, "rb") as f:
        return f.read(n)


def copy(src, dst):
    shutil.copyfile(src, dst)
    os.chmod(dst, 0o644)
    return dst


def kernel(step, tools, attr, work):
    keys.kernel_public_halves()
    elf = copy(os.path.join(nix_build(step, f"{attr}.kernel"), "nonos-kernel"), os.path.join(work, "kernel.elf"))
    trailer = os.path.join(work, "kernel.zk_trailer.bin")
    step.run(tools.enroll, "kernel", elf, KERNEL_ROOT, trailer)
    step.run(tools.enroll, "verify-kernel", KERNEL_ROOT, elf, trailer)
    keys.stage(KERNEL_ROOT, KERNEL_ROOT + ".transcript")
    return elf, trailer


def loader(step, tools, attr, work):
    efi = copy(os.path.join(nix_build(step, f"{attr}.loader"), "nonos_boot.efi"), os.path.join(work, "BOOTX64.EFI"))
    trailer = os.path.join(work, "bootloader.trailer")
    step.run(tools.enroll, "bootloader", efi, LOADER_ROOT, trailer)
    step.run(tools.enroll, "verify-bootloader", LOADER_ROOT, efi, trailer)
    keys.stage(LOADER_ROOT, LOADER_ROOT + ".transcript")
    return efi, trailer


def sign_kernel(step, tools, elf, trailer, rollback, work, path_only=False):
    keys.require([keys.SIGNING_KEY, keys.MLDSA_PREFIX + ".seed"], "the kernel signature")
    signed, attested = os.path.join(work, "kernel_signed.bin"), os.path.join(work, "kernel.bin")
    step.run(tools.sign_kernel, "--key", keys.SIGNING_KEY, "--input", elf, "--output", signed,
             "--mldsa65-key", keys.MLDSA_PREFIX + ".seed", "--mldsa65-pub", keys.MLDSA_PREFIX + ".pub",
             "--rollback-index", rollback, "--verify")
    step.run(tools.embed, "--input", signed, "--output", attested, "--proof-file", trailer, "--verbose",
             *(["--path-only"] if path_only else []))
    step.run("python3", "scripts/check_staged_kernel.py", "--elf", elf, "--staged", attested)
    return attested


def records(step, rollback, work):
    """boot_root.approval, which the kernel holds the loader to, and
    kernel.approval, under which the TPM releases the device secret. Both are
    signed with the device policy key; the first is committed so the release
    names exactly what was signed."""
    current = os.path.isfile(BOOT_ROOT_RECORD) and head(BOOT_ROOT_RECORD) == head(LOADER_ROOT)
    if not current:
        keys.require([keys.DEVICE_POLICY], "the boot-root record")
        step.run("python3", "tools/nonos-policy-approve", "boot-root", "--root", LOADER_ROOT,
                 "--epoch", rollback, "--key", keys.DEVICE_POLICY, "--out", BOOT_ROOT_RECORD)
        keys.stage(BOOT_ROOT_RECORD)
    approval = os.path.join(work, "kernel.approval")
    if not os.path.isfile(keys.DEVICE_POLICY):
        say("  no device policy key: no kernel.approval, so the TPM keeps the device secret sealed")
        return BOOT_ROOT_RECORD, None
    step.run("python3", "tools/nonos-policy-approve", "approve", "--transcript", KERNEL_ROOT + ".transcript",
             "--root", KERNEL_ROOT, "--key", keys.DEVICE_POLICY, "--out", approval)
    return BOOT_ROOT_RECORD, approval


def secure_boot(step, efi, required):
    """Signs the loader for UEFI Secure Boot with the db key. The signature
    sits in the certificate table, which the enrolled Authenticode digest
    leaves out, so the enrollment above still holds."""
    if not (os.path.isfile(keys.DB_KEY) and os.path.isfile(keys.DB_CERT)):
        if required:
            sys.exit(f"  this loader requires Secure Boot and there is no {keys.DB_KEY}")
        say("  no Secure Boot db key: the loader is not signed for Secure Boot")
        return efi
    signed = efi + ".signed"
    # osslsigncode refuses to overwrite its output, and a seal run again
    # finds the last run's signed loader here.
    if os.path.lexists(signed):
        os.remove(signed)
    if platform.system() == "Darwin":
        step.run("osslsigncode", "sign", "-certs", keys.DB_CERT, "-key", keys.DB_KEY, "-h", "sha256",
                 "-in", efi, "-out", signed)
        step.run("osslsigncode", "verify", "-CAfile", keys.DB_CERT, "-in", signed)
    else:
        step.run("sbsign", "--key", keys.DB_KEY, "--cert", keys.DB_CERT, "--output", signed, efi)
        step.run("sbverify", "--cert", keys.DB_CERT, signed)
    return signed
