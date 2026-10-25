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

"""A release seals the capsules as make declares them, never the development
set (tools/nix/capsules.nix, bySlugDev). This reads the set that was built,
not the flag that chose it: the development set's link farm is
nonos-capsules-dev, and each capsule it builds again is
nonos-capsule-<slug>-dev, so a release handed either is refused whatever
capsules.DEVELOPMENT says."""

import os
import sys


def store_name(path):
    """The name a store path was built under, its hash cut off."""
    return os.path.basename(os.path.realpath(path)).split("-", 1)[-1]


def dev_builds(capsules_out):
    """What in this capsule set is a development build: the set itself when it
    is the development one, and each capsule built with its dev features."""
    found = ["the set itself"] if store_name(capsules_out) == "nonos-capsules-dev" else []
    for slug in sorted(os.listdir(capsules_out)):
        if store_name(os.path.join(capsules_out, slug)) == f"nonos-capsule-{slug}-dev":
            found.append(slug)
    return found


def refuse(capsules_out, release):
    """Stop a release seal handed the development set, before it signs or
    checks anything against it."""
    if not release:
        return
    found = dev_builds(capsules_out)
    if found:
        sys.exit(f"  a release never seals the development capsule set; {capsules_out} has development "
                 f"builds: {', '.join(found)}")
