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

"""Seed a target's corpus with the committed capsule manifests or NONOS-ID
certificates, the bytes the spawn gate reads on a shipping image."""

import argparse
import pathlib
import shutil

TRUST = pathlib.Path(__file__).resolve().parents[3] / "nonos-data/trust/capsules"
SUFFIX = {"manifest": ".manifest.bin", "id_cert": ".nonos_id_cert.bin"}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="corpus/<target>")
    out = pathlib.Path(ap.parse_args().out)
    files = sorted(TRUST.glob("*" + SUFFIX[out.name]))
    out.mkdir(parents=True, exist_ok=True)
    for f in files:
        shutil.copy(f, out / f.name)
    print(f"wrote {len(files)} seeds to {out}")


if __name__ == "__main__":
    main()
