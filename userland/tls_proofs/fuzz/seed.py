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

"""Write the seed corpus for tls_cert: the re-signing gateway's chain as a TLS
Certificate message body, the shape verify_chain reads, and every fixture
certificate on its own, so the fuzzer starts deep inside the DER readers."""

import argparse
import pathlib

FIX = pathlib.Path(__file__).resolve().parent.parent / "src"
CERTS = ["fixtures/gw0.der", "fixtures/gw1.der", "fixtures/gw2.der",
         "fixtures/p384_authority.der", "fixtures/p384_many_names.der",
         "example_leaf.der", "example_ca.der", "relay_cert.der"]


def body(ders):
    items = b"".join(len(d).to_bytes(3, "big") + d + b"\0\0" for d in ders)
    return b"\0" + len(items).to_bytes(3, "big") + items


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", default="corpus/tls_cert", help="corpus directory")
    out = pathlib.Path(ap.parse_args().out)
    out.mkdir(parents=True, exist_ok=True)
    ders = [(FIX / c).read_bytes() for c in CERTS]
    (out / "seed-gateway-chain").write_bytes(body(ders[:3]))
    for name, d in zip(CERTS, ders):
        (out / f"seed-{pathlib.Path(name).stem}").write_bytes(d)
        (out / f"seed-{pathlib.Path(name).stem}-body").write_bytes(body([d]))
    print(f"wrote {1 + 2 * len(ders)} seeds to {out}")


if __name__ == "__main__":
    main()
