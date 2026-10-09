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
"""The rows that need no boot: every proof crate, every static check, and the
kernel, loader and host crates compiling and testing. Each is a list of steps;
a row passes when every step exits 0."""

import glob
import os

KEY = "NONOS_SIGNING_KEY={k}/check_seed RUSTUP_TOOLCHAIN=nightly-2026-01-16"
KF = "--release --target x86_64-nonos.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem"
UEFI = ("NONOS_SIGNING_KEY={k}/check_seed NONOS_MLDSA65_PUBKEY={k}/k_mldsa.pub "
        "NONOS_KERNEL_ATTEST_ROOT={k}/root32 RUSTUP_TOOLCHAIN=nightly-2026-01-16 "
        "RUSTFLAGS='-C panic=abort -C target-feature=+crt-static --cfg curve25519_dalek_backend=\"serial\"' "
        "cargo check --target x86_64-unknown-uefi --release --features {p}")
HOST = ["nonos-attest-path", "nonos-boot-measure", "nonos-bootloader/tools/embed-trailer",
        "nonos-bootloader/tools/sign-kernel", "nonos-sign", "nonos-verify", "security/nonos-secops",
        "nonos-stark-enroll"]


def proof_crates(tree):
    dirs = sorted(d for d in glob.glob(os.path.join(tree, "userland", "*_proofs")) if os.path.isfile(d + "/Cargo.toml"))
    return [("RUSTFLAGS= cargo test --release", d) for d in dirs]


# Checks that read a built kernel or capsule, run by the image targets that
# hand them one; on a bare tree they only print their usage.
ON_ARTIFACTS = {"check_declared_caps.py", "check_kernel_elf.py", "check_staged_kernel.py"}
# Checks whose source-only mode is a flag.
ARGS = {"check_aarch64_boot.py": " --self-test"}


def static_checks(tree):
    found = sorted(glob.glob(os.path.join(tree, "scripts", "check_*.py")))
    picked = [s for s in found if os.path.basename(s) not in ON_ARTIFACTS]
    steps = [(f"python3 {s}{ARGS.get(os.path.basename(s), '')}", tree) for s in picked]
    if os.path.isfile(os.path.join(tree, "nonos-ci", "run-static-checks.sh")):
        steps.append(("bash nonos-ci/run-static-checks.sh", tree))
    return steps


def crates(tree, keys):
    steps = [(f"env {KEY.format(k=keys)} cargo check {KF} --no-default-features --features microkernel-core", tree)]
    for p in ("standard-qemu", "production", "dev-qemu"):
        steps.append((UEFI.format(k=keys, p=p), os.path.join(tree, "nonos-bootloader")))
    for d in HOST:
        if os.path.isfile(os.path.join(tree, d, "Cargo.toml")):
            steps.append(("RUSTFLAGS= cargo test --release", os.path.join(tree, d)))
    return steps


ROWS = {"proof-crates": proof_crates, "static-checks": static_checks, "crates": crates}
