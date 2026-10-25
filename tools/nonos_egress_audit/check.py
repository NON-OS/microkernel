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

"""Holding what the source can reach against what privacy.md lists, both
ways: an unlisted capsule or literal is a new path, and a listed one the
source no longer has is a page that no longer matches the code."""


def findings(capsules, literals, listed):
    out = []
    for name in sorted(set(capsules) - listed["capsules"]):
        out.append(f"{name} holds Network and privacy.md does not list it")
    for name in sorted(listed["capsules"] - set(capsules)):
        out.append(f"privacy.md lists {name}, which holds no Network")
    for lit in sorted(set(literals) - listed["places"]):
        out.append(f"{lit} at {literals[lit][0]} is not listed in privacy.md")
    for lit in sorted(listed["places"] - set(literals)):
        out.append(f"privacy.md lists {lit}, which no networked source names")
    return out
