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
"""check_driver_scope.py's two sides: a capsule's discover ids, its row's ranges."""

import re

BROKER = "src/hardware/broker"
CAPSULES = "userland/capsule_driver_"


def item(text, name):
    """The text after `const name ... =` up to its `;`, or `fn name`'s body."""
    text = re.sub(r"//[^\n]*", "", text)
    m = re.search(r"\b(const|fn)\s+" + name + r"\b", text)
    if not m:
        return None
    depth = 0
    for i in range(m.end(), len(text)):
        depth += text[i] in "[{(" and 1 or 0
        depth -= text[i] in "]})" and 1 or 0
        if depth == 0 and text[i] == (";" if m.group(1) == "const" else "}"):
            body = text[m.end():i + 1]
            return body.split("=", 1)[1] if m.group(1) == "const" else body
    return None


def capsule_ids(body):
    """Each `0xA..=0xB` and each lone hex literal, as (lo, hi)."""
    pair = r"(0x[0-9A-Fa-f]+)\s*\.\.=\s*(0x[0-9A-Fa-f]+)"
    out = [(int(a, 16), int(b, 16)) for a, b in re.findall(pair, body)]
    rest = re.sub(pair, "", body)
    return out + [(int(h, 16),) * 2 for h in re.findall(r"\b0x[0-9A-Fa-f]+\b", rest)]


def row_ranges(root, service):
    """The (lo, hi) ranges of every Ids entry in `service`'s row, or None."""
    files = ["scope_table.rs", "scope_rows.rs", "scope_hosts.rs", "scope_ids.rs", "scope_ids_nic.rs", "scope_lpss.rs"]
    text = "\n".join((root / BROKER / f).read_text() for f in files)
    m = re.search(r'\("' + re.escape(service) + r'",\s*([^\n]*)\),', text)
    if not m:
        return None
    row = m.group(1)
    for _ in range(4):
        row = re.sub(r"\b([A-Z][A-Z0-9_]+)\b", lambda n: item(text, n.group(1)) or n.group(1), row)
    flat = [int(h, 16) for ids in re.findall(r"Ids\([^,]+,\s*&\[([^\]]*)\]", row)
            for h in re.findall(r"0x[0-9A-Fa-f]+", ids)]
    return list(zip(flat[0::2], flat[1::2]))
