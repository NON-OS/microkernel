#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Writes onion_descriptor.txt: a v3 onion service descriptor built the way
# the fork's hs_descriptor.c encodes one (cert layout, both encrypted layers,
# NUL padding to a multiple of 10000, the prefixed signature), from fixed
# throwaway test seeds that sign nothing real. The Rust decoder is held to
# this file and to the values printed below.
#
# Run: python3 onion_descriptor.py > onion_descriptor.expect

import base64, hashlib, struct
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives import serialization

def pub(sk):
    return sk.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)

def ed(seed):
    return Ed25519PrivateKey.from_private_bytes(seed)

def cert(kind, certified, signer, expiry_hours=600000):
    body = bytes([1, kind]) + struct.pack(">I", expiry_hours) + bytes([1]) + certified
    body += bytes([1]) + struct.pack(">H", 32) + bytes([4, 0]) + pub(signer)
    return body + signer.sign(body)

def pem(kind, data):
    b = base64.b64encode(data).decode()
    lines = [b[i:i + 64] for i in range(0, len(b), 64)]
    return "-----BEGIN %s-----\n%s\n-----END %s-----\n" % (kind, "\n".join(lines), kind)

def b64(data):
    return base64.b64encode(data).decode().rstrip("=")

def layer(plain, secret, subcred, revision, salt, constant):
    shake = hashlib.shake_256(secret + subcred + struct.pack(">Q", revision) + salt + constant)
    keys = shake.digest(80)
    key, iv, mac_key = keys[:32], keys[32:48], keys[48:]
    enc = Cipher(algorithms.AES(key), modes.CTR(iv)).encryptor()
    body = enc.update(plain) + enc.finalize()
    mac = hashlib.sha3_256(struct.pack(">Q", 32) + mac_key + struct.pack(">Q", 16) + salt + body).digest()
    return salt + body + mac

def pad(text):
    data = text.encode()
    return data + b"\0" * (-len(data) % 10000)

blinded = ed(bytes(range(32)))
signing = ed(bytes([0x11] * 32))
auth = ed(bytes([0x22] * 32))
subcred = bytes([0x55] * 32)
revision = 42
B = pub(blinded)

specs = bytes([3, 0, 6, 1, 2, 3, 4]) + struct.pack(">H", 9001)
specs += bytes([2, 20]) + bytes([0xAA] * 20) + bytes([3, 32]) + bytes([0xBB] * 32)
point = (
    "introduction-point %s\n" % base64.b64encode(specs).decode()
    + "onion-key ntor %s\n" % b64(bytes([0x66] * 32))
    + "auth-key\n" + pem("ED25519 CERT", cert(0x09, pub(auth), signing))
    + "enc-key ntor %s\n" % b64(bytes([0x77] * 32))
    + "enc-key-cert\n" + pem("ED25519 CERT", cert(0x0B, bytes([0x44] * 32), signing))
)
inner = "create2-formats 2\n" + point

def build(inner, name):
    encrypted = layer(pad(inner), B, subcred, revision, bytes([0x01] * 16), b"hsdir-encrypted-data")

    middle = "desc-auth-type x25519\ndesc-auth-ephemeral-key %s\n" % b64(bytes([0x88] * 32))
    for i in range(16):
        middle += "auth-client %s %s %s\n" % (b64(bytes([i] * 8)), b64(bytes([i] * 16)), b64(bytes([i] * 16)))
    middle += "encrypted\n" + pem("MESSAGE", encrypted)
    superencrypted = layer(pad(middle), B, subcred, revision, bytes([0x02] * 16), b"hsdir-superencrypted-data")

    doc = (
        "hs-descriptor 3\ndescriptor-lifetime 180\n"
        + "descriptor-signing-key-cert\n" + pem("ED25519 CERT", cert(0x08, pub(signing), blinded))
        + "revision-counter %d\n" % revision
        + "superencrypted\n" + pem("MESSAGE", superencrypted)
    )
    sig = signing.sign(b"Tor onion service descriptor sig v3" + doc.encode())
    doc += "signature %s\n" % b64(sig)
    open(name, "w").write(doc)
    return doc

doc = build(inner, "onion_descriptor.txt")
# The same descriptor from a service under load: hs_descriptor.c writes the
# pow-params line after create2-formats, the seed in padded base64 and the
# expiry with format_iso_time_nospace.
pow_seed = bytes([0xAA] * 32)
build("create2-formats 2\n" + "pow-params v1 %s 250 2026-10-03T12:00:00\n" % base64.b64encode(pow_seed).decode()
      + point, "onion_descriptor_pow.txt")

print("blinded", B.hex())
print("subcredential", subcred.hex())
print("auth_key", pub(auth).hex())
print("length", len(doc))
