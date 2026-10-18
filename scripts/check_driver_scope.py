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
"""Fail when a driver capsule's discover table names a device id its kernel
scope row (src/hardware/broker/scope_table.rs, K6f) does not admit, so the
kernel's table and the capsules' own cannot drift. Rows by class are pinned
in kernel_proofs::driver_scope."""

import argparse
import sys
from pathlib import Path

from driver_scope_rows import CAPSULES, DISCOVER, capsule_ids, item, row_ranges


def drift(root, discover=DISCOVER):
    out = []
    for service, path, names in discover:
        row = row_ranges(root, service)
        if row is None:
            out.append(f"{service}: no row in scope_table.rs")
            continue
        text = (root / (CAPSULES + path)).read_text()
        for name in names.split():
            body = item(text, name)
            if body is None:
                out.append(f"{service}: {path} has no {name}; update DISCOVER")
                continue
            for lo, hi in capsule_ids(body):
                if not any(a <= lo and hi <= b for a, b in row):
                    out.append(f"{service}: {name} names {lo:#06x}..={hi:#06x}, outside its row")
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        from driver_scope_selftest import self_test

        self_test()
        return 0
    bad = drift(a.root)
    for line in bad:
        print(f"driver scope: {line}")
    print(f"driver scope: {len(DISCOVER)} discover tables checked, {len(bad)} outside their row")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
