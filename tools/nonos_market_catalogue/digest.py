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
"""The digest every listing names its bytes by."""

import subprocess
import sys


def blake3(data: bytes) -> str:
    """BLAKE3-256, the hash every other artifact in this tree is named by.

    b3sum is what the signing tools use, so the digest in a listing is
    the same one a person gets checking the artifact by hand.
    """
    try:
        done = subprocess.run(
            ["b3sum", "--no-names", "--raw"],
            input=data, stdout=subprocess.PIPE, check=True,
        )
    except FileNotFoundError:
        sys.exit("b3sum not found: install it, or the digests would be guesses")
    return done.stdout[:32].hex()
