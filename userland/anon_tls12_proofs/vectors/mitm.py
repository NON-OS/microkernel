#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# A man in the middle, written apart from the client: it relays the real
# relay's ServerHello, Certificate and CertificateRequest from
# chacha_certreq.server, but puts its own ECDHE key in the
# ServerKeyExchange, keeping the relay's signature (which covers another
# key), and then finishes the handshake as a server would, with keys it can
# compute because the ECDHE key is its own. The client's ClientHello and
# key share are deterministic under the replay seed, so they are read from
# chacha_certreq.client.
#
# A client that checks the ServerKeyExchange signature refuses before it
# sends anything more. One that does not completes a session with this
# script. Writes mitm.server.

import hashlib, hmac, os, struct
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives import serialization

HERE = os.path.dirname(os.path.abspath(__file__))
server = open(os.path.join(HERE, "chacha_certreq.server"), "rb").read()
client = open(os.path.join(HERE, "chacha_certreq.client"), "rb").read()

def records(b):
    out, at = [], 0
    while at + 5 <= len(b):
        n = struct.unpack(">H", b[at + 3:at + 5])[0]
        out.append((b[at], b[at + 5:at + 5 + n]))
        at += 5 + n
    return out

def messages(b):
    stream = b"".join(body for kind, body in records(b) if kind == 22)
    out, at = [], 0
    while at + 4 <= len(stream):
        n = int.from_bytes(stream[at + 1:at + 4], "big")
        out.append(stream[at:at + 4 + n])
        at += 4 + n
    return out

def prf(secret, label, seed, n):
    out, a = b"", hmac.new(secret, label + seed, hashlib.sha256).digest()
    while len(out) < n:
        out += hmac.new(secret, a + label + seed, hashlib.sha256).digest()
        a = hmac.new(secret, a, hashlib.sha256).digest()
    return out[:n]

def record(kind, body):
    return bytes([kind, 3, 3]) + struct.pack(">H", len(body)) + body

# The client's hello and key share, as the replay seed makes them.
client_plain = [body for kind, body in records(client) if kind == 22]
hello = messages(client)[0]
client_random = hello[6:38]
cke = [m for m in messages(client) if m[0] == 16][0]
client_point = cke[5:70]

# The relay's flight, with the ServerKeyExchange's key replaced.
relay = [m for m in messages(server)]
sh = relay[0]
server_random = sh[6:38]
attacker = ec.derive_private_key(0x1337C0FFEE, ec.SECP256R1())
point = attacker.public_key().public_bytes(serialization.Encoding.X962, serialization.PublicFormat.UncompressedPoint)
forged = []
for m in relay:
    if m[0] == 12:
        body = m[4:]
        body = body[:4] + point + body[4 + 65:]
        m = m[:4] + body
    forged.append(m)
flight = record(22, b"".join(forged))

# The client's flight as it would be sent: empty Certificate, its key share.
empty_cert = bytes([11, 0, 0, 3, 0, 0, 0])
transcript = hello + b"".join(forged) + empty_cert + cke
shared = attacker.exchange(ec.ECDH(), ec.EllipticCurvePublicKey.from_encoded_point(ec.SECP256R1(), client_point))
master = prf(shared, b"extended master secret", hashlib.sha256(transcript).digest(), 48)
block = prf(master, b"key expansion", server_random + client_random, 88)
client_key, server_key, client_iv, server_iv = block[:32], block[32:64], block[64:76], block[76:88]
client_finished = bytes([20, 0, 0, 12]) + prf(master, b"client finished", hashlib.sha256(transcript).digest(), 12)
transcript += client_finished
server_finished = bytes([20, 0, 0, 12]) + prf(master, b"server finished", hashlib.sha256(transcript).digest(), 12)
nonce = bytes(a ^ b for a, b in zip(server_iv, bytes(4) + (0).to_bytes(8, "big")))
aad = (0).to_bytes(8, "big") + bytes([22, 3, 3]) + struct.pack(">H", len(server_finished))
sealed = ChaCha20Poly1305(server_key).encrypt(nonce, server_finished, aad)
open(os.path.join(HERE, "mitm.server"), "wb").write(flight + record(20, b"\x01") + record(22, sealed))
print("mitm.server", len(flight), "bytes of flight")
