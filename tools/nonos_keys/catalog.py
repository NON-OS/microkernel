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
"""Every key the build uses: what it is, where its private half lives, where its
public half goes, and how it is made. Paths are the ones the make files read."""

import subprocess
from dataclasses import dataclass


@dataclass(frozen=True)
class Key:
    name: str
    kind: str  # "ed25519", "mldsa65", "seed32", "p256", "secureboot"
    private: str
    public: str


def capsule_prefixes(root):
    out = subprocess.run(["python3", "tools/nonos-capsule-key-prefixes", "--root", root],
                         capture_output=True, text=True, check=True).stdout.split()
    if not out:
        raise SystemExit("tools/nonos-capsule-key-prefixes named no capsule")
    return out


def pair(name, prefix, pub_dir):
    return [Key(f"{name} {alg}", alg, f".keys/{prefix}_{alg}.seed", f"{pub_dir}/{prefix}_{alg}.pub")
            for alg in ("ed25519", "mldsa65")]


def every_key(root="."):
    keys = pair("trust anchor", "nonos_trust_anchor", "nonos-data/trust/keys")
    for p in capsule_prefixes(root):
        keys += pair(f"publisher {p}", f"{p}_publisher", "nonos-data/trust/keys")
    # The Linux programs NONOS ships, guests of the personality and not
    # capsules, so no Capsule.mk names them: the userland in the store and
    # the Qwen runner, under one publisher.
    keys += pair("publisher linux userland", "linux_userland_publisher", "nonos-data/trust/keys")
    keys += [
        Key("kernel signing ed25519", "seed32", "nonos-bootloader/keys/signing_key_v1.bin",
            "nonos-bootloader/keys/signing_key_v1.pub"),
        Key("kernel signing mldsa65", "mldsa65", "nonos-bootloader/keys/kernel_mldsa65.seed",
            "nonos-bootloader/keys/kernel_mldsa65.pub"),
        Key("market operator ed25519", "seed32", ".keys/marketplace_operator_ed25519.seed",
            ".keys/marketplace_operator_ed25519.pub"),
        Key("device policy p256", "p256", ".keys/device_policy_p256.pem",
            "nonos-data/trust/policy/device_policy_p256.pub"),
    ]
    for sb in ("PK", "KEK", "db"):
        keys.append(Key(f"secure boot {sb}", "secureboot", f".keys/secureboot/{sb}.key",
                        f".keys/secureboot/{sb}.crt"))
    return keys
