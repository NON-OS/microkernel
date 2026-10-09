#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# The hs-ntor known answer the Rust client is held to: the formulas of the
# fork's src/test/hs_ntor_ref.py over fixed throwaway X25519 scalars, plus
# the virtual hop key expansion (hs_ntor_circuit_key_expansion) and one
# INTRODUCE1 cell built as hs_cell.c builds it, with and without a proof of
# work extension. Prints name value lines.

import hashlib, struct
from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey, X25519PublicKey
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes

PROTOID = b"tor-hs-ntor-curve25519-sha3-256-1"
T_HSENC = PROTOID + b":hs_key_extract"
T_HSVERIFY = PROTOID + b":hs_verify"
T_HSMAC = PROTOID + b":hs_mac"
M_HSEXPAND = PROTOID + b":hs_key_expand"

def mac(k, m):
    return hashlib.sha3_256(struct.pack("!q", len(k)) + k + m).digest()

def raw(pk):
    return pk.public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)

def priv(byte):
    return X25519PrivateKey.from_private_bytes(bytes([byte] * 32))

x, b, y = priv(0x31), priv(0x32), priv(0x33)
X, B, Y = raw(x.public_key()), raw(b.public_key()), raw(y.public_key())
auth = bytes([0x41] * 32)
subcred = bytes([0x42] * 32)

dh_bx = x.exchange(X25519PublicKey.from_public_bytes(B))
dh_yx = x.exchange(X25519PublicKey.from_public_bytes(Y))
secret = dh_bx + auth + X + B + PROTOID
keys = hashlib.shake_256(secret + T_HSENC + M_HSEXPAND + subcred).digest(64)
enc_key, mac_key = keys[:32], keys[32:]

rend = dh_yx + dh_bx + auth + B + X + Y + PROTOID
seed = mac(rend, T_HSENC)
verify = mac(rend, T_HSVERIFY)
auth_mac = mac(verify + auth + B + Y + X + PROTOID + b"Server", T_HSMAC)
expanded = hashlib.shake_256(seed + M_HSEXPAND).digest(128)

# INTRODUCE1, as hs_cell_build_introduce1 lays it out.
cookie = bytes([0x51] * 20)
rp_key = bytes([0x52] * 32)
specs = bytes([2, 0, 6, 10, 0, 0, 1]) + struct.pack(">H", 9001) + bytes([2, 20]) + bytes([0x53] * 20)
header = bytes(20) + bytes([2]) + struct.pack(">H", 32) + auth + bytes([0])
plain = cookie + bytes([0]) + bytes([1]) + struct.pack(">H", 32) + rp_key + specs
if len(header) + len(plain) < 246:
    plain += bytes(246 - len(header) - len(plain))
enc = Cipher(algorithms.AES(enc_key), modes.CTR(bytes(16))).encryptor()
ct = enc.update(plain) + enc.finalize()
cell = header + X + ct
cell += mac(mac_key, cell)

for name, value in [("X", X), ("B", B), ("Y", Y), ("dh_bx", dh_bx), ("dh_yx", dh_yx),
                    ("enc_key", enc_key), ("mac_key", mac_key), ("seed", seed),
                    ("auth_mac", auth_mac), ("expanded", expanded), ("introduce1", cell)]:
    print(name, value.hex())

# The same INTRODUCE1 carrying a proof-of-work solution as its one extension
# (trn_cell_extension_pow: version, nonce, effort, seed head, solution). The
# field body is test_hs_pow.c's encoded_hex for its high-effort vector.
pow_body = bytes.fromhex("01" "59217255555555555555555555555555" "000f4240" "aaaaaaaa"
                         "0f3db97b9cac20c1771680a1a34848d3")
plain = cookie + bytes([1, 0x02, len(pow_body)]) + pow_body
plain += bytes([1]) + struct.pack(">H", 32) + rp_key + specs
if len(header) + len(plain) < 246:
    plain += bytes(246 - len(header) - len(plain))
enc = Cipher(algorithms.AES(enc_key), modes.CTR(bytes(16))).encryptor()
cell = header + X + enc.update(plain) + enc.finalize()
cell += mac(mac_key, cell)
print("introduce1_pow", cell.hex())

# The virtual hop: one BEGIN to the service sealed by the client (SHA3
# seeded with Df, AES-256-CTR under Kf, zero IV), and one CONNECTED from the
# service sealed the other way (Db, Kb), as relay_crypto does.
def relay_cell(command, stream, body):
    cell = bytearray(509)
    cell[0] = command
    cell[3:5] = struct.pack(">H", stream)
    cell[9:11] = struct.pack(">H", len(body))
    cell[11:11 + len(body)] = body
    return cell

def seal(cell, seed, key):
    h = hashlib.sha3_256(seed + bytes(cell)).digest()
    cell[5:9] = h[:4]
    enc = Cipher(algorithms.AES(key), modes.CTR(bytes(16))).encryptor()
    return enc.update(bytes(cell)) + enc.finalize(), h[:20]

df, db, kf, kb = expanded[:32], expanded[32:64], expanded[64:96], expanded[96:]
begin, _ = seal(relay_cell(1, 1, b":80\0"), df, kf)
connected, seen = seal(relay_cell(4, 1, b""), db, kb)
print("hop_begin", begin.hex())
print("hop_connected", connected.hex())
print("hop_connected_digest", seen.hex())
