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
"""Where each file of a tier is served: the NONOS model repository first,
then the Qwen team's own file on Hugging Face as the fallback.

The repository's layout is fixed: <base>/<tier>/<file>, the tier word and
the file name exactly as the signed pins give them, so a mirror is the tree
`mirror` writes, served as it is by any HTTPS server. The base is what the
operator passes as --nonos-mirror; no hostname is assumed.
"""

import re
import sys

from .download import url as upstream

# What the fetcher's URL parser (userland/nonos_http) takes: HTTPS, a host of
# letters, digits, dots and hyphens with no port, a path of printable ASCII.
BASE = re.compile(r"https://[A-Za-z0-9.-]+(/[\x21-\x7e]*)?")


def check_base(base):
    """The base without its trailing slash; exits on one the fetcher refuses."""
    if not BASE.fullmatch(base) or "?" in base or "#" in base:
        sys.exit(f"--nonos-mirror {base!r}: give https://host[/path], no port, query or space")
    return base.rstrip("/")


def nonos(base, tier, name):
    return f"{base}/{tier}/{name}"


def mirrors(base, tier, name):
    """Every URL for `name`, the NONOS repository first when there is one."""
    return ([nonos(base, tier, name)] if base else []) + [upstream(name)]
