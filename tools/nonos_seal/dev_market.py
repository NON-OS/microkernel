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

"""A development image's market index: the release catalogue with the
schema 3 test listings in tools/devnet/market added, so its Marketplace both
installs real apps and lists priced entries whose paid paths can be walked.
All of it is signed with the throwaway operator key nonos-dev-image made in
its copy, which only that copy's market capsule trusts. A release never reads
the test listings (scripts/check_dev_catalogue.py)."""

import os

from . import capsules, keys

SOURCE = "tools/devnet/market/index.json"
DEV_TREE = os.path.join("target", "dev", "tree")


def in_dev_tree():
    """Whether this is nonos-dev-image's copy: a repository of its own at
    <checkout>/target/dev/tree, never the checkout that holds the real keys."""
    return os.path.realpath(os.getcwd()).endswith(os.sep + DEV_TREE) and os.path.isdir(".git")


def chosen():
    return capsules.DEVELOPMENT and in_dev_tree()


def extra():
    """The index tool's arguments that add the test listings."""
    with open(keys.OPERATOR_PUB, "rb") as f:
        key = f.read()
    # The checkout's operator key signing fake listings would make them ones a
    # release trusts; the copy's key is a throwaway, so the two must differ.
    checkout = os.path.join(os.getcwd(), "..", "..", "..", keys.OPERATOR_PUB)
    if os.path.isfile(checkout) and open(checkout, "rb").read() == key:
        raise SystemExit("  the development catalogue is never signed with the checkout's operator key")
    return ["--extra", SOURCE]
