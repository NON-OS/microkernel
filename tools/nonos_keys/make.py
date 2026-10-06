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
"""Making one key. Private halves are written under umask 077 into 0700
directories and end at mode 0600; no tool output is shown, so nothing a
generator prints, a seed included, reaches the terminal."""

import os
import shutil
import subprocess


def quiet(cmd):
    r = subprocess.run(cmd, capture_output=True)
    if r.returncode != 0:
        raise SystemExit(f"{cmd[0]} {cmd[1]} failed (exit {r.returncode}); its output is withheld")
    return r.stdout


def ed25519_public(seed):
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
    from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat

    return Ed25519PrivateKey.from_private_bytes(seed).public_key().public_bytes(Encoding.Raw, PublicFormat.Raw)


def write_new(path, data, mode):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, mode)
    with os.fdopen(fd, "wb") as f:
        f.write(data)


def make(root, key, capsule_sign):
    priv, pub = os.path.join(root, key.private), os.path.join(root, key.public)
    for d in {os.path.dirname(priv), os.path.dirname(pub)}:
        os.makedirs(d, mode=0o700, exist_ok=True)
    os.chmod(os.path.dirname(priv), 0o700)
    old = os.umask(0o077)
    try:
        if key.kind in ("ed25519", "mldsa65"):
            prefix = priv[: -len(".seed")]
            quiet([capsule_sign, "keygen", "--alg", key.kind, "--out", prefix])
            if prefix + ".pub" != pub:
                shutil.move(prefix + ".pub", pub)
        elif key.kind == "seed32":
            seed = os.urandom(32)
            write_new(priv, seed, 0o600)
            write_new(pub, ed25519_public(seed), 0o644)
        elif key.kind == "p256":
            quiet(["openssl", "ecparam", "-name", "prime256v1", "-genkey", "-noout", "-out", priv])
            write_new(pub, quiet(["openssl", "ec", "-in", priv, "-pubout", "-outform", "DER"])[-64:], 0o644)
        elif key.kind == "secureboot":
            cn = f"/CN=NONOS owner {os.path.basename(priv)[:-4]}/"
            quiet(["openssl", "req", "-new", "-x509", "-newkey", "rsa:2048", "-nodes", "-sha256",
                   "-days", "3650", "-subj", cn, "-keyout", priv, "-out", pub])
            quiet(["openssl", "x509", "-in", pub, "-outform", "DER", "-out", pub[:-4] + ".cer"])
    finally:
        os.umask(old)
    os.chmod(priv, 0o600)
    os.chmod(pub, 0o644)
