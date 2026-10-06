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

"""Where ek's keys live, and the rules around them. The seal reads a private
key only by handing its path to the tool that signs with it; it never prints,
copies or writes one, and before it stages anything it checks that git would
not pick up a private file."""

import os
import subprocess
import sys

TRUST = "nonos-data/trust"

# The layout tools/nonos-key-ceremony makes (tools/nonos_keys/catalog.py).
SIGNING_KEY = os.environ.get("SIGNING_KEY", "nonos-bootloader/keys/signing_key_v1.bin")
MLDSA_PREFIX = os.environ.get("KERNEL_MLDSA65_PREFIX", "nonos-bootloader/keys/kernel_mldsa65")
TA_SEEDS = {alg: f".keys/nonos_trust_anchor_{alg}.seed" for alg in ("ed25519", "mldsa65")}
TA_PUBS = {alg: f"{TRUST}/keys/nonos_trust_anchor_{alg}.pub" for alg in ("ed25519", "mldsa65")}
DEVICE_POLICY = os.environ.get("DEVICE_POLICY_KEY", ".keys/device_policy_p256.pem")
OPERATOR_SEED = ".keys/marketplace_operator_ed25519.seed"
OPERATOR_PUB = ".keys/marketplace_operator_ed25519.pub"
DB_KEY, DB_CERT = ".keys/secureboot/db.key", ".keys/secureboot/db.crt"

# The public halves of the kernel signing keys, where the flake's bootloader
# build reads them (tools/nix/image.nix).
KERNEL_ED25519_PUB = f"{TRUST}/keys/kernel_signing_ed25519.pub"
KERNEL_MLDSA65_PUB = f"{TRUST}/keys/kernel_mldsa65.pub"

SECRET = (".seed", ".pem", ".key", ".secret")


def require(paths, why):
    missing = [p for p in paths if not os.path.isfile(p)]
    if missing:
        sys.exit(f"  {why} needs " + ", ".join(missing) + " (tools/nonos-key-ceremony makes them)")


def private(path):
    if path.endswith(SECRET) or path.startswith((".keys/", "nonos-bootloader/keys/")):
        return True
    return "/keys/" in path and not path.endswith(".pub")


def guard():
    """Fails if git would track a private key: the rule every commit keeps."""
    out = subprocess.run(["git", "status", "--short", "--untracked-files=all"],
                         capture_output=True, text=True, check=True).stdout
    bad = [line for line in out.splitlines() if private(line[3:])]
    if bad:
        sys.exit("  git would track private key material:\n    " + "\n    ".join(bad))


def stage(*paths):
    guard()
    subprocess.run(["git", "add", "--", *paths], check=True)


def kernel_public_halves():
    """Writes the public halves of the kernel signing keys into the tree, so
    the bootloader the flake builds checks the kernel these keys sign."""
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
    from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat

    require([SIGNING_KEY, MLDSA_PREFIX + ".seed", MLDSA_PREFIX + ".pub"], "the kernel signature")
    with open(SIGNING_KEY, "rb") as f:
        seed = f.read(32)
    public = Ed25519PrivateKey.from_private_bytes(seed).public_key().public_bytes(Encoding.Raw, PublicFormat.Raw)
    with open(KERNEL_ED25519_PUB, "wb") as f:
        f.write(public)
    with open(MLDSA_PREFIX + ".pub", "rb") as src, open(KERNEL_MLDSA65_PUB, "wb") as dst:
        dst.write(src.read())
    stage(KERNEL_ED25519_PUB, KERNEL_MLDSA65_PUB)
