#!/usr/bin/env python3
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
"""Make the package-authentication vectors in Alpine's own layout.

An index is two gzip members, a signature tar and the index tar; a package
is three: a signature, a control member holding .PKGINFO, and the data. The
signature and control tars are cut before their end-of-archive blocks, as
abuild-tar --cut leaves them, and each signature is PKCS#1 v1.5 over SHA-1
of the member it covers. The key is a throwaway, not Alpine's.
"""
import argparse
import base64
import gzip
import hashlib
import io
import subprocess
import tarfile
from pathlib import Path

NAMED = "alpine-devel@lists.alpinelinux.org-6165ee59.rsa.pub"


def tar(entries, cut):
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.USTAR_FORMAT) as t:
        for name, data in entries:
            info = tarfile.TarInfo(name)
            info.size, info.uname, info.gname = len(data), "root", "root"
            t.addfile(info, io.BytesIO(data))
    raw = buf.getvalue()
    while cut and raw.endswith(b"\0" * 512):
        raw = raw[:-512]
    return raw


def gz(data):
    return gzip.compress(data, mtime=0)


def signed(key, data, name=NAMED):
    sig = subprocess.run(["openssl", "dgst", "-sha1", "-sign", str(key)],
                         input=data, capture_output=True, check=True).stdout
    return gz(tar([(".SIGN.RSA." + name, sig)], True)) + data


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--key", type=Path, required=True, help="PEM RSA private key")
    ap.add_argument("--out", type=Path, default=Path(__file__).parent)
    a = ap.parse_args()
    data = gz(tar([("usr/bin/hello", b"\x7fELF a test payload\n")], False))
    other = gz(tar([("usr/bin/hello", b"\x7fELF something else\n")], False))
    info = f"pkgname = hello\npkgver = 1.0-r0\ndatahash = {hashlib.sha256(data).hexdigest()}\n"
    control = gz(tar([(".PKGINFO", info.encode())], True))
    (a.out / "hello.apk").write_bytes(signed(a.key, control) + data)
    (a.out / "swapped.apk").write_bytes(signed(a.key, control) + other)
    sum_ = "Q1" + base64.b64encode(hashlib.sha1(control).digest()).decode()
    record = f"C:{sum_}\nP:hello\nV:1.0-r0\nA:x86_64\n\n".encode()
    index = gz(tar([("DESCRIPTION", b"test index"), ("APKINDEX", record)], False))
    (a.out / "APKINDEX.tar.gz").write_bytes(signed(a.key, index))
    (a.out / "untrusted.tar.gz").write_bytes(signed(a.key, index, "someone-else.rsa.pub"))
    der = subprocess.run(["openssl", "rsa", "-in", str(a.key), "-pubout", "-outform", "DER"],
                         capture_output=True, check=True).stdout
    (a.out / "test_key.spki").write_bytes(der)


if __name__ == "__main__":
    main()
