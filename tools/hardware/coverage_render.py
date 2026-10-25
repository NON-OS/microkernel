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

"""One area's page of the coverage map."""

TITLES = {
    "net": "Wired Ethernet", "wifi": "Wi-Fi", "storage": "Storage", "usb": "USB",
    "audio": "Audio", "input": "Input, EC and battery", "platform": "Platform and IOMMU",
    "gpu": "Graphics",
}
COMMON = {"1": "most machines", "2": "common", "3": "servers or niche"}


def ids(text):
    return " ".join(f"`{i}`" for i in text.split()) if text != "-" else "-"


def page(area, rows):
    rows = sorted(rows, key=lambda r: (r["priority"].replace("-", "P9"), r["common"]))
    out = [
        f"# Coverage: {TITLES.get(area, area)}",
        "",
        "Written by `tools/hardware/coverage.py` from `coverage-parts.tsv`; edit that file,",
        "not this page. [How to read it](coverage.md#how-to-read-it).",
        "",
        "| Priority | Part | Years | How common | Ids | Named by the driver | In Standard |"
        " Owner | Gap |",
        "|---|---|---|---|---|---|---|---|---|",
    ]
    for r in rows:
        out.append(
            f"| {r['priority']} | {r['part']} | {r['years']} | {COMMON[r['common']]} | "
            f"{ids(r['ids'])} | {r['matched']} | {r['image']} | {r['owner']} | {r['gap']} |"
        )
    return "\n".join(out) + "\n"
