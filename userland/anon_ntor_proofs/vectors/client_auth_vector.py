#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Client authorization known answer, by the fork's build_descriptor_cookie_keys
# and decrypt_descriptor_cookie: KEYS = SHAKE-256(subcredential | x25519(sk, pk),
# 40), CLIENT-ID = KEYS[0:8], COOKIE-KEY = KEYS[8:40], the cookie AES-256-CTR
# under COOKIE-KEY with the entry's IV. Fixed throwaway scalars.

import base64, hashlib
from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes

def raw(k):
    return k.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)

client = X25519PrivateKey.from_private_bytes(bytes([0x61] * 32))
ephemeral = X25519PrivateKey.from_private_bytes(bytes([0x62] * 32))
subcred = bytes([0x63] * 32)
cookie = bytes(range(16))
iv = bytes([0x64] * 16)
seed = client.exchange(ephemeral.public_key())
keys = hashlib.shake_256(subcred + seed).digest(40)
enc = Cipher(algorithms.AES(keys[8:40]), modes.CTR(iv)).encryptor()
encrypted = enc.update(cookie) + enc.finalize()
b64 = lambda b: base64.b64encode(b).decode().rstrip("=")
print("seed", seed.hex())
print("ephemeral", raw(ephemeral).hex())
print("client_id", keys[:8].hex())
print("encrypted", encrypted.hex())
print("cookie", cookie.hex())
print("line", b64(keys[:8]), b64(iv), b64(encrypted))
print("secret_b32", base64.b32encode(bytes([0x61] * 32)).decode().rstrip("=").lower())
