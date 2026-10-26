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
"""The seal's choice of the development catalogue, run: chosen() holds for a
development seal in a copy at target/dev/tree with a repository of its own,
and for nothing else, and that copy refuses to sign with the key the
checkout beside it holds."""

import os
import tempfile

from dev_catalogue_content import inside

PUB = os.path.join(".keys", "marketplace_operator_ed25519.pub")


def choices(dev_market, capsules, top):
    tree = os.path.join(top, "target", "dev", "tree")
    bare = os.path.join(top, "bare", "target", "dev", "tree")
    plain = os.path.join(top, "plain")
    for d in (os.path.join(tree, ".git"), bare, os.path.join(plain, ".git")):
        os.makedirs(d)
    out = []
    for where, development, want in ((tree, True, True), (tree, False, False), (plain, True, False),
                                     (plain, False, False), (bare, True, False)):
        capsules.DEVELOPMENT = development
        with inside(where):
            if dev_market.chosen() != want:
                out.append(f"chosen() is {not want} for a {'development' if development else 'release'} "
                           f"seal in {os.path.relpath(where, top)}")
    return out, tree


def same_key(dev_market, top, tree):
    """The copy's key written as the checkout's: the seal must stop first."""
    for d in (top, tree):
        os.makedirs(os.path.join(d, ".keys"), exist_ok=True)
        with open(os.path.join(d, PUB), "wb") as f:
            f.write(bytes(32))
    with inside(tree):
        try:
            dev_market.extra()
        except SystemExit:
            return []
    return ["the development catalogue was signed with the checkout's operator key"]


def behaviour(dev_market, capsules):
    with tempfile.TemporaryDirectory() as top:
        try:
            out, tree = choices(dev_market, capsules, top)
            return out + same_key(dev_market, top, tree)
        finally:
            capsules.DEVELOPMENT = False
