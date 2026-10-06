#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Known answers for the TLS 1.2 client's own primitives, from Python's
# hashlib and hmac and the cryptography package: SHA-384 across its block
# and padding boundaries, HMAC-SHA384 with short and long keys, the TLS 1.2
# PRF (RFC 5246 5) over SHA-256 and SHA-384, and the SubjectPublicKeyInfo of
# the certificate in each recording. Prints hash.expect.

import hashlib, hmac, os, struct
from cryptography import x509
from cryptography.hazmat.primitives import serialization

HERE = os.path.dirname(os.path.abspath(__file__))

def prf(h, secret, label, seed, n):
    out, a = b"", hmac.new(secret, label + seed, h).digest()
    while len(out) < n:
        out += hmac.new(secret, a + label + seed, h).digest()
        a = hmac.new(secret, a, h).digest()
    return out[:n]

for n in [0, 1, 3, 55, 111, 112, 113, 127, 128, 129, 255, 256, 1000]:
    msg = bytes((i * 7 + 3) & 0xff for i in range(n))
    print("sha384", msg.hex() or "-", hashlib.sha384(msg).hexdigest())
for key_len in [0, 20, 128, 131]:
    key = bytes((i * 13 + 1) & 0xff for i in range(key_len))
    msg = b"what do ya want for nothing?" * 3
    print("hmac384", key.hex() or "-", msg.hex(), hmac.new(key, msg, hashlib.sha384).hexdigest())
for name, h in [("prf256", hashlib.sha256), ("prf384", hashlib.sha384)]:
    for n in [12, 48, 88, 100]:
        secret = bytes(range(48))
        seed = bytes(range(100, 164))
        print(name, secret.hex(), b"key expansion".hex(), seed.hex(), n, prf(h, secret, b"key expansion", seed, n).hex())

def leaf(path):
    b = open(path, "rb").read()
    stream, at = b"", 0
    while at + 5 <= len(b):
        n = struct.unpack(">H", b[at + 3:at + 5])[0]
        if b[at] == 22:
            stream += b[at + 5:at + 5 + n]
        if b[at] == 20:
            break
        at += 5 + n
    at = 0
    while True:
        kind, n = stream[at], int.from_bytes(stream[at + 1:at + 4], "big")
        if kind == 11:
            body = stream[at + 4:at + 4 + n]
            first = int.from_bytes(body[3:6], "big")
            return body[6:6 + first]
        at += 4 + n

for case in ["chacha_certreq", "aes_gcm_renegotiate", "close_notify"]:
    cert = x509.load_der_x509_certificate(leaf(os.path.join(HERE, case + ".server")))
    spki = cert.public_key().public_bytes(serialization.Encoding.DER, serialization.PublicFormat.SubjectPublicKeyInfo)
    print("spki", case, spki.hex())
