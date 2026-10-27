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
"""The sha256 upstream publishes for one release asset, read from its
checksum file. SimpleX writes `_sha256sums` beside each release's assets
(scripts/simplex-chat-reproduce-builds.sh in its repository): sha256sum's
lines, `<hex>  <tag>/<asset>`, signed in `_sha256sums.asc` by the release
key whose fingerprint docs/REPRODUCE.md gives (FINGERPRINT). The file's
signature is checked before it is read (tools/nonos-source-asset)."""

import re

# SimpleX Build (Key for signing releases) <build@simplex.chat>, as
# docs/REPRODUCE.md in simplex-chat names it; the key is pinned beside it.
FINGERPRINT = "BBDF7BDAD1548B16836AF5B9D53BDFD153C366BA"
KEY = "userland/capsule_market/keys/simplex-build.asc"
LINE = re.compile(r"([0-9a-f]{64}) [ *](\S+)")


def sum_of(text: str, asset: str) -> str:
    """The one sha256 the file gives `asset` by the last part of its path,
    or ValueError when it gives none, or two that differ."""
    found = set()
    for line in text.splitlines():
        m = LINE.fullmatch(line.rstrip("\r"))
        if m and m.group(2).rsplit("/", 1)[-1] == asset:
            found.add(m.group(1))
    if len(found) != 1:
        raise ValueError(f"the checksum file gives {asset} {len(found)} sha256s, not one")
    return found.pop()


def signer(status: str) -> str:
    """The primary key fingerprint of a good signature in gpgv's
    --status-fd output, or "" when there is none."""
    for line in status.splitlines():
        parts = line.split()
        if parts[:2] == ["[GNUPG:]", "VALIDSIG"] and len(parts) >= 12:
            return parts[11]
    return ""
