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


"""What builds the copy before the boots."""


def prepare(names):
    """The copy and its keys, and the selftest image when it boots."""
    if "capsule" in names:
        return ["python3", "tools/nonos-dev-image", "--profile", "selftest"]
    return ["python3", "tools/nonos-dev-image", "--keys-only"]
