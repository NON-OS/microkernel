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
"""Seed nsh_roles' fuzz targets with valid role tables and revocation lists."""

import argparse
import hashlib
import pathlib
from struct import pack

SIG = b"\xe5" * (64 + 3309)
def b8(x): return bytes([len(x)]) + x
def caps(names): return bytes([len(names)]) + b"".join(b8(n.encode()) for n in sorted(names))
def kid(k): return hashlib.sha256(k).digest()
def key(fill): return bytes([fill]) * (32 + 1952)

def keys(fills):
    ks = sorted((key(f) for f in fills), key=kid)
    return bytes([len(ks)]) + b"".join(ks)

def role(name, flags, approvals, session, ceiling, escalation):
    head = b8(name) + bytes([flags, approvals]) + pack("<Q", session)
    return head + caps(ceiling) + caps(escalation)

ROLES = [
    role(b"auditor", 0, 2, 8 * 3600000, ["AttestRead", "CoreExec", "FileSystem", "IPC"], []),
    role(b"deploy", 0, 1, 3600000, ["AppInstall", "CoreExec", "IPC"], ["AppInstall", "Network"]),
    role(b"operator", 1, 1, 8 * 3600000, ["CoreExec", "IPC", "Memory"], ["Network", "StoreWrite"]),
]

def table(roles=ROLES, auth=(0x41, 0x42), appr=(0x51, 0x52, 0x53)):
    body = b"NSHP" + pack("<HH", 1, 0) + b"\xf1" * 16
    body += pack("<QQQ", 5, 1_800_000_000_000, 24 * 3600000)
    body += keys(auth) + keys(appr) + bytes([len(roles)]) + b"".join(roles)
    return body + kid(key(auth[0])) + SIG

def revoke(ids):
    body = b"NSHV" + pack("<HH", 1, 0) + b"\xf1" * 16 + pack("<Q", 1_800_000_000_000)
    body += pack("<H", len(ids)) + b"".join(sorted(ids))
    return body + kid(key(0x41)) + SIG

SEEDS = {
    "nsh_policy": [table(), table(ROLES[:1], (0x41,), ())],
    "nsh_revoke": [
        revoke([]),
        revoke([b"\x11" * 16, b"\x99" * 16]),
        revoke([bytes([i]) * 16 for i in range(1, 40)]),
    ],
}

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="corpus", help="corpus root directory")
    root = pathlib.Path(ap.parse_args().out)
    for target, seeds in SEEDS.items():
        (root / target).mkdir(parents=True, exist_ok=True)
        for i, seed in enumerate(seeds):
            (root / target / f"seed{i}").write_bytes(seed)
    print(f"wrote {sum(map(len, SEEDS.values()))} seeds under {root}")

if __name__ == "__main__":
    main()
