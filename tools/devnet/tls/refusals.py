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
"""Certificates the throwaway root's key could sign and nonos_tls must still
refuse, for tls_proofs. The root's key is public, so these are what anyone
can make; none is served. Their keys are derived from labels and not
written: no test signs with them."""

from mint import CLIENT, DER, HOST, SERVER, cert, key, name, serves

OTHER = "rpc.mainnet.example"


def refusals(rn, rk):
    """Each file and its bytes, for a root named rn whose key is rk."""
    def leaf(label, serial, cn, exts, ca=False, issuer=rn, signer=rk):
        return label, cert(name(cn), issuer, key(label).public_key(), signer, serial, ca, exts)

    ik, inn = key("inter"), name("NONOS-devnet-throwaway-inter")
    made = [
        # A name other than the local RPC's, which is all the root may vouch for.
        leaf("other-leaf", 3, OTHER, serves(OTHER, SERVER)),
        # The right name, but not for a server, or for no stated use.
        leaf("client-leaf", 4, HOST, serves(HOST, CLIENT)),
        leaf("noeku-leaf", 5, HOST, serves(HOST)),
        # The right name and use, but a CA: a leaf that could sign others.
        leaf("ca-leaf", 6, HOST, serves(HOST, SERVER), ca=True),
        # An intermediate under a root whose path length is zero, and its leaf.
        ("inter", cert(inn, rn, ik.public_key(), rk, 7, True)),
        leaf("inter-leaf", 8, HOST, serves(HOST, SERVER), issuer=inn, signer=ik),
    ]
    return {f"{label}.der": c.public_bytes(DER) for label, c in made}
