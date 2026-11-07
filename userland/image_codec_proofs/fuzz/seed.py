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

"""Seed image_decode with the real images the decode proofs start from. A
fuzzer seldom builds a PNG's chunks or a JPEG's markers on its own; each seed
is the op byte image_decode reads first (PNG 0, BMP 1, GIF 2, JPEG 3, raw LZ4
4), then a whole image of that kind."""

import argparse
import pathlib

USERLAND = pathlib.Path(__file__).resolve().parents[2]
KINDS = {".png": 0, ".bmp": 1, ".gif": 2, ".jpg": 3}
DIRS = [
    "image_paint_proofs/fixtures/pngsuite",
    "image_paint_proofs/fixtures/misc",
    "toolkit/image_tests/tests/fixtures",
    "capsule_browser_proofs/fixtures/img",
]
# No image larger than libFuzzer's default input cap is a useful seed.
MAX_SEED = 64 * 1024


def raw():
    """The 2 x 2 raw frame the decode proofs seed with (decode_fuzz_tests.rs)."""
    pixels = bytes((b * 37) & 0xFF for b in range(16))
    return (2).to_bytes(4, "little") + (2).to_bytes(4, "little") + pixels


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="corpus/image_decode")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    n = 0
    for d in DIRS:
        for f in sorted((USERLAND / d).glob("*")):
            op = KINDS.get(f.suffix.lower())
            if op is None or f.stat().st_size > MAX_SEED:
                continue
            (out / f"{op}-{f.name}").write_bytes(bytes([op]) + f.read_bytes())
            n += 1
    (out / "4-raw").write_bytes(bytes([4]) + raw())
    print(f"wrote {n + 1} seeds to {out}")


if __name__ == "__main__":
    main()
