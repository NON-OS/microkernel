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
"""How generate.py derives a key and builds a certificate. A key comes from a
fixed public label and ECDSA signs deterministically, so the bytes never
change between runs."""

import datetime as dt
import hashlib

from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization as ser
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.x509.oid import ExtendedKeyUsageOID, NameOID

HOST = "rpc.devnet.nonos.test"
N = 0xFFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551
DER, P8, CLEAR = ser.Encoding.DER, ser.PrivateFormat.PKCS8, ser.NoEncryption()
KU = dict(content_commitment=False, key_encipherment=False, data_encipherment=False,
          key_agreement=False, crl_sign=False, encipher_only=False, decipher_only=False)
SERVER, CLIENT = ExtendedKeyUsageOID.SERVER_AUTH, ExtendedKeyUsageOID.CLIENT_AUTH


def key(label):
    seed = hashlib.sha256(f"NONOS devnet throwaway {label}, public on purpose".encode()).digest()
    return ec.derive_private_key(int.from_bytes(seed, "big") % (N - 1) + 1, ec.SECP256R1())


def name(cn):
    return x509.Name([x509.NameAttribute(NameOID.COMMON_NAME, cn)])


def cert(cn, issuer, pub, signer, serial, ca, exts=()):
    b = (x509.CertificateBuilder().subject_name(cn).issuer_name(issuer).public_key(pub).serial_number(serial)
         .not_valid_before(dt.datetime(2026, 1, 1, tzinfo=dt.UTC)).not_valid_after(dt.datetime(2045, 12, 31, tzinfo=dt.UTC))
         .add_extension(x509.BasicConstraints(ca=ca, path_length=0 if ca else None), True)
         .add_extension(x509.KeyUsage(digital_signature=not ca, key_cert_sign=ca, **KU), True))
    for ext in exts:
        b = b.add_extension(ext, False)
    return b.sign(signer, hashes.SHA256(), ecdsa_deterministic=True)


def serves(host, *usages):
    """The extensions of a leaf for host: the usages it is for, if any, then its name."""
    eku = [x509.ExtendedKeyUsage(list(usages))] if usages else []
    return eku + [x509.SubjectAlternativeName([x509.DNSName(host)])]
