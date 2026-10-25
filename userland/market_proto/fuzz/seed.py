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

"""Write the seed corpus for linux_policy: one reply of each kind as the
market writes it (OP_GET_LINUX_POLICY v2), so the fuzzer starts inside both
readers' accepting paths."""

import argparse
import pathlib

HASH = bytes(range(32))
ARG = bytes(i % 251 for i in range(320))  # policy | listing_key | release_key


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="corpus/linux_policy", help="corpus directory")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    for name, body in (("none", bytes([2, 0]) + HASH), ("refused", bytes([2, 2]) + HASH),
                       ("policy", bytes([2, 1]) + HASH + ARG)):
        (out / name).write_bytes(body)


if __name__ == "__main__":
    main()
