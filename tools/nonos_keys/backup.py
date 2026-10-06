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
"""An encrypted offline copy of every private half: a tar of .keys and
nonos-bootloader/keys under AES-256-GCM with a key from scrypt over a passphrase
read from the terminal. Written only outside the working tree, and restored
only where nothing would be overwritten."""

import getpass
import io
import os
import tarfile

MAGIC = b"NONOSBK1"
DIRS = (".keys", "nonos-bootloader/keys")


def _key(passphrase, salt):
    from cryptography.hazmat.primitives.kdf.scrypt import Scrypt

    return Scrypt(salt=salt, length=32, n=1 << 17, r=8, p=1).derive(passphrase.encode())


def _passphrase(confirm):
    p = getpass.getpass("backup passphrase: ")
    if len(p) < 16:
        raise SystemExit("the passphrase must be at least 16 characters")
    if confirm and getpass.getpass("again: ") != p:
        raise SystemExit("the passphrases differ")
    return p


def backup(root, out):
    if os.path.commonpath([os.path.abspath(out), os.path.abspath(root)]) == os.path.abspath(root):
        raise SystemExit("the backup must be written outside the working tree")
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM

    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w") as t:
        for d in DIRS:
            if os.path.isdir(os.path.join(root, d)):
                t.add(os.path.join(root, d), arcname=d)
    salt, nonce = os.urandom(16), os.urandom(12)
    sealed = AESGCM(_key(_passphrase(True), salt)).encrypt(nonce, buf.getvalue(), MAGIC)
    fd = os.open(out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, "wb") as f:
        f.write(MAGIC + salt + nonce + sealed)


def restore(root, src):
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM

    blob = open(src, "rb").read()
    if blob[:8] != MAGIC:
        raise SystemExit(f"{src} is not a NONOS key backup")
    salt, nonce = blob[8:24], blob[24:36]
    from cryptography.exceptions import InvalidTag

    try:
        plain = AESGCM(_key(_passphrase(False), salt)).decrypt(nonce, blob[36:], MAGIC)
    except InvalidTag:
        raise SystemExit("wrong passphrase, or the backup was altered") from None
    with tarfile.open(fileobj=io.BytesIO(plain)) as t:
        for m in t.getmembers():
            if not m.name.startswith(DIRS) or ".." in m.name or (m.isfile() and os.path.exists(os.path.join(root, m.name))):
                raise SystemExit(f"refusing to restore {m.name}: outside the key dirs or already present")
        t.extractall(root, filter="data")
