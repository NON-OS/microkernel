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
"""Check the enroll tool's PE Authenticode digest against pesign and osslsigncode.

For each image, signed with a throwaway sbsign key made here and deleted after:
our digest of the signed file, pesign's of the unsigned one, and osslsigncode's
computed and embedded digests must be one value. Our digest of the unsigned file
must be that value too, or the tool must refuse it: an image not 8-byte aligned
with bytes past its sections is padded by signing and changes digest, so it is
never enrolled under a value the signed loader would not produce.
"""

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path


def run(*cmd):
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout


def digests(enroll, image, tmp):
    unsigned = subprocess.run([enroll, "authenticode", image], capture_output=True, text=True)
    pesign = run("pesign", "-h", "-i", image).split()[0]
    signed = str(Path(tmp) / "signed.efi")
    run("sbsign", "--key", f"{tmp}/k.pem", "--cert", f"{tmp}/c.pem", "--output", signed, image)
    ours_signed = run(enroll, "authenticode", signed).strip()
    out = subprocess.run(["osslsigncode", "verify", "-in", signed], capture_output=True, text=True).stdout
    found = dict(re.findall(r"(Current|Calculated) message digest\s*:\s*([0-9A-Fa-f]+)", out))
    agreed = [ours_signed, pesign, found.get("Calculated", ""), found.get("Current", "")]
    return [x.lower() for x in agreed], unsigned.stdout.strip() if unsigned.returncode == 0 else None


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--enroll", required=True, help="the nonos-stark-enroll binary")
    ap.add_argument("images", nargs="+", help="PE images, unsigned")
    args = ap.parse_args()
    bad = 0
    with tempfile.TemporaryDirectory() as tmp:
        run("openssl", "req", "-new", "-x509", "-newkey", "rsa:2048", "-nodes", "-days", "1",
            "-subj", "/CN=throwaway authenticode check", "-keyout", f"{tmp}/k.pem", "-out", f"{tmp}/c.pem")
        for image in args.images:
            agreed, unsigned = digests(args.enroll, image, tmp)
            one = len(set(agreed)) == 1 and len(agreed[0]) == 64
            ok = one and unsigned in (agreed[0], None)
            how = "signed and unsigned" if unsigned else "signed; unsigned refused, signing pads it"
            bad += not ok
            print(f"{'ok  ' if ok else 'FAIL'} {agreed[0]} {image} ({how})" + ("" if ok else f" {agreed}"))
    print(f"{len(args.images) - bad} of {len(args.images)} images agree with pesign and osslsigncode")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
