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
this file. The keys come from fixed public labels and ECDSA signs
deterministically, so every run writes the same bytes; --check proves the
committed files are these.    tools/devnet/tls/generate.py [--check]"""

import argparse
import datetime as dt
import hashlib
from pathlib import Path

from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization as ser
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.x509.oid import ExtendedKeyUsageOID, NameOID

HERE, HOST = Path(__file__).resolve().parent, "rpc.devnet.nonos.test"
N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
DER, P8, CLEAR = ser.Encoding.DER, ser.PrivateFormat.PKCS8, ser.NoEncryption()
KU = dict(content_commitment=False, key_encipherment=False, data_encipherment=False,
          key_agreement=False, crl_sign=False, encipher_only=False, decipher_only=False)


def key(label):
    seed = hashlib.sha256(f"NONOS devnet throwaway {label}, public on purpose".encode()).digest()
    return ec.derive_private_key(int.from_bytes(seed, "big") % (N - 1) + 1, ec.SECP256R1())


def cert(cn, issuer, pub, signer, serial, ca, exts=()):
    b = (x509.CertificateBuilder().subject_name(cn).issuer_name(issuer).public_key(pub).serial_number(serial)
         .not_valid_before(dt.datetime(2026, 1, 1, tzinfo=dt.UTC)).not_valid_after(dt.datetime(2045, 12, 31, tzinfo=dt.UTC))
         .add_extension(x509.BasicConstraints(ca=ca, path_length=0 if ca else None), True)
         .add_extension(x509.KeyUsage(digital_signature=not ca, key_cert_sign=ca, **KU), True))
    for ext in exts:
        b = b.add_extension(ext, False)
    return b.sign(signer, hashes.SHA256(), ecdsa_deterministic=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    rk, lk = key("root"), key("leaf")
    rn, ln = (x509.Name([x509.NameAttribute(NameOID.COMMON_NAME, cn)]) for cn in ("NONOS-devnet-throwaway-root", HOST))
    root = cert(rn, rn, rk.public_key(), rk, 1, True)
    leaf = cert(ln, rn, lk.public_key(), rk, 2, False, [x509.ExtendedKeyUsage([ExtendedKeyUsageOID.SERVER_AUTH]),
                                                       x509.SubjectAlternativeName([x509.DNSName(HOST)])])
    spki = rk.public_key().public_bytes(DER, ser.PublicFormat.SubjectPublicKeyInfo)
    out = {"root.der": root.public_bytes(DER), "root_subject.der": rn.public_bytes(), "root_spki.der": spki,
           "leaf.der": leaf.public_bytes(DER), "root-key.throwaway.der": rk.private_bytes(DER, P8, CLEAR),
           "leaf-key.throwaway.der": lk.private_bytes(DER, P8, CLEAR)}
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
