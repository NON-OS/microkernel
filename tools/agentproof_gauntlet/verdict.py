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
"""A Gauntlet run's serial log, read and judged.

The lines it reads are written by the image, never by the agent: `[BOX]` and
`[BOX-MISSING]` by the Terminal from the box's keyed frames, `[AGENTPROOF]` by
the headless Terminal, the IOMMU line by the kernel. The agent's own output
arrives only as `[AGENTPROOF-OUT] ` rows, which are never read as anything else.
"""

import re

KINDS = ("write", "network", "exec", "jit", "display", "models", "call", "barred", "unserved")


def fields(line: str) -> dict:
    return dict(t.split("=", 1) for t in line.split()[1:] if "=" in t)


def read(log: str) -> dict:
    r = {"status": None, "box": None, "boxes": 0, "missing": None, "out": [], "iommu": None,
         "net": None}
    for line in log.splitlines():
        if line.startswith("[AGENTPROOF-OUT] "):
            r["out"].append(line[len("[AGENTPROOF-OUT] "):])
        elif line.startswith("[AGENTPROOF] v=1 "):
            r["status"] = fields(line)
        elif line.startswith("[BOX] v=1 "):
            r["box"], r["boxes"] = fields(line), r["boxes"] + 1
        elif line.startswith("[BOX-NET] v=1 "):
            r["net"] = fields(line)
        elif line.startswith("[BOX-MISSING] v=1 "):
            r["missing"] = fields(line)
        elif m := re.search(r"\[IOMMU\] .*enforcing=(\d), unconfined grants=(\d+)", line):
            r["iommu"] = {"enforcing": m.group(1) == "1", "unconfined": int(m.group(2))}
    return r


def judge(r: dict, max_refused: int = 0) -> list:
    """Why the run fails; an empty list is a pass."""
    why = []
    if not r["status"] or r["status"].get("status") != "done":
        why.append(f"the run did not finish: {r['status'] or 'no [AGENTPROOF] line'}")
    if not r["iommu"] or not r["iommu"]["enforcing"] or r["iommu"]["unconfined"]:
        why.append("the IOMMU was not enforcing with 0 unconfined grants")
    if r["missing"] or r["boxes"] != 1:
        why.append(f"not one whole box report: {r['missing'] or str(r['boxes']) + ' [BOX] lines'}")
        return why
    box = r["box"]
    if box.get("preset") == "learn" or box.get("enforced") == "no":
        why.append("a learn run is not enforced: run in the file it learned")
    if box.get("started") != "yes":
        why.append("the program never started")
    refused = int(box.get("refused", "0"))
    if refused > max_refused:
        kinds = ", ".join(f"{k} {box[k]}" for k in KINDS if box.get(k, "0") != "0")
        why.append(f"the box refused {refused} actions ({kinds}); at most {max_refused} allowed")
    return why
