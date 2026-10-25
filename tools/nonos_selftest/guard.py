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


"""What the self-test refuses before it runs anything: a boot that would
read a key in this checkout. A make lane signs with the keys beside it, so
one run here would need .keys/; every make boot runs in the copy, with the
copy's throwaway keys, or not at all."""

import os

from .boots import DEV_IMAGE, TREE


def refusal(boots, root, built):
    """Why this plan may not run, or None."""
    tree = os.path.realpath(os.path.join(root, TREE))
    for b in boots:
        if b.cmd[0] == "make" and os.path.realpath(b.cwd) != tree:
            return f"{b.name} would run make in {b.cwd}, which signs with the keys there"
        key = b.env.get("NONOS_SIGNING_KEY")
        if key and not os.path.realpath(key).startswith(tree + os.sep):
            return f"{b.name} would sign with {key}, outside the copy"
    if built:
        return None
    if any(b.cmd[0] == "make" for b in boots) and not os.path.isfile(os.path.join(tree, ".keys/signing_key_v1.bin")):
        return f"{TREE} has no throwaway keys yet; run without --no-build to make them there"
    if any(b.name == "capsule" for b in boots) and not os.path.isfile(os.path.join(root, DEV_IMAGE)):
        return f"no selftest image at {DEV_IMAGE}; run without --no-build to seal it"
    return None
