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
"""The local test chain's throwaway TLS root and its one leaf (P5), beside
this file, with the certificates under it tls_proofs proves are refused
(refusals.py). The keys come from fixed public labels and ECDSA signs
deterministically, so every run writes the same bytes; --check proves the
committed files are these.    tools/devnet/tls/generate.py [--check]"""

import argparse
import hashlib
from pathlib import Path

from cryptography.hazmat.primitives import serialization as ser

from mint import CLEAR, DER, HOST, P8, SERVER, cert, key, name, serves
from refusals import refusals

HERE = Path(__file__).resolve().parent


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    rk, lk = key("root"), key("leaf")
    rn = name("NONOS-devnet-throwaway-root")
    root = cert(rn, rn, rk.public_key(), rk, 1, True)
    leaf = cert(name(HOST), rn, lk.public_key(), rk, 2, False, serves(HOST, SERVER))
    spki = rk.public_key().public_bytes(DER, ser.PublicFormat.SubjectPublicKeyInfo)
    out = {"root.der": root.public_bytes(DER), "root_subject.der": rn.public_bytes(), "root_spki.der": spki,
           "leaf.der": leaf.public_bytes(DER), "root-key.throwaway.der": rk.private_bytes(DER, P8, CLEAR),
           "leaf-key.throwaway.der": lk.private_bytes(DER, P8, CLEAR), **refusals(rn, rk)}
    if args.check:
        stale = [n for n, b in out.items() if not (HERE / n).is_file() or (HERE / n).read_bytes() != b]
        print(f"devnet-tls: {'stale: ' + ' '.join(stale) if stale else 'the committed files are these'}")
        return 1 if stale else 0
    for n, b in out.items():
        (HERE / n).write_bytes(b)
    print(f"devnet-tls: root SPKI sha256 {hashlib.sha256(spki).hexdigest()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
