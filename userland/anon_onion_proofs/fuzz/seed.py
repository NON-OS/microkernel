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

"""Write seed corpora for the net.anon fuzz targets from the proofs' own
vectors, so the fuzzer starts inside real documents: descriptors (signed,
with a proof-of-work line, and the fork's bad-signature one), consensus and
microdescriptor documents, a names list, cells and HTTP answers."""

import argparse
import pathlib

HERE = pathlib.Path(__file__).resolve().parent
NTOR = HERE.parent.parent / "anon_ntor_proofs" / "vectors"


def write(out, name, data):
    out.mkdir(parents=True, exist_ok=True)
    (out / name).write_bytes(data)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", required=True, help="directory to hold one corpus per target")
    root = pathlib.Path(ap.parse_args().out)

    d = root / "anon_descriptor"
    for f in ["onion_descriptor.txt", "onion_descriptor_pow.txt", "tor_desc_bad_sig.txt"]:
        write(d, f, (NTOR / f).read_bytes())
    write(d, "inner", b"create2-formats 2\npow-params v1 qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqo= 250 "
          b"2026-10-03T12:00:00\nintroduction-point AwAGAQIDBCMpAhSqqqqqqqqqqqqqqqqqqqqqqqqqqgMgu7u7\n")
    write(d, "auth", b"desc-auth-type x25519\ndesc-auth-ephemeral-key iIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIg\n"
          b"auth-client AAAAAAAAAAA AAAAAAAAAAAAAAAAAAAAAA AAAAAAAAAAAAAAAAAAAAAA\n")
    write(d, "key", b"iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd:descriptor:x25519:"
          b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")

    n = root / "anon_names"
    write(n, "list", b"anyone-hosts-version 1\nanyone-hosts-status signed\npublished 2026-10-01 00:00:00\n"
          b"valid-until 2026-11-01 00:00:00\nlander.anyone iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone\n"
          b"anyone-hosts-signature gadmrvl67444hgzrhsnhzknxaimfnzp6az3wq4d2j7hrf7th34elrrad.anyone\n"
          b"-----BEGIN SIGNATURE-----\n" + b"A" * 64 + b"\n" + b"A" * 22 + b"==\n-----END SIGNATURE-----\n")
    write(n, "http", b"HTTP/1.0 200 OK\r\n\r\n" + (n / "list").read_bytes())

    w = root / "anon_network"
    for f in ["consensus-microdesc.txt", "microdescs.txt"]:
        write(w, f, (NTOR / f).read_bytes())
    write(w, "versions", bytes([0, 0, 7, 0, 4, 0, 4, 0, 5]))
    write(w, "relay", bytes([0, 0, 0, 1, 3]) + bytes([2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]) + bytes(498))
    write(w, "ack", bytes([0, 0, 0]))
    write(w, "rendezvous2", bytes(64))
    write(w, "hsdir", b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\nhs-descriptor 3\n")


main()
