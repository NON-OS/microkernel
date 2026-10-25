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
"""The boot session's verdict on the catalogue:

    PYTHONPATH=tools python3 -m nonos_market_catalogue.verdict [--write] APP=LOG ...

An app passes when each of its logs (one per debug run of the design notes/
05; several unite) shows the install start in its release's box, no policy
refusal, the workload ending well (a process exiting status 0, none exiting
otherwise or on a signal), and exactly one measured set, the app's own.
--write puts each passing set into its recipe. Exit 0 when every app passes."""

import json
import sys
from pathlib import Path

from nonos_linux_pack.syscalls import market_set

from .verdict_run import run_verdict

RECIPES = Path("userland/capsule_market/linux-catalogue.json")


def verdict(app: str, logs: list):
    """(ok, calls hex or "", why) for every run of one app; their sets unite."""
    sets = []
    for at, log in enumerate(logs, 1):
        ok, got, why = run_verdict(app, log)
        if not ok:
            return False, "", f"run {at}: {why}"
        sets += got
    return True, market_set(sets), f"{len(logs)} run(s): installed in its box, workload ran, set measured"


def write(found: dict) -> None:
    doc = json.loads(RECIPES.read_text())
    for app in doc["apps"]:
        if app["package"] in found:
            app["calls"] = found[app["package"]]
    apps = ",\n".join("  " + json.dumps(a) for a in doc["apps"])
    RECIPES.write_text('{"format": ' + json.dumps(doc["format"]) + ', "apps": [\n' + apps + "\n]}\n")


def main(argv) -> int:
    pairs = [a.split("=", 1) for a in argv if a != "--write"]
    if not pairs or any(len(p) != 2 for p in pairs):
        return print(__doc__, file=sys.stderr) or 2
    logs = {}
    for app, log in pairs:  # several logs of one app are its runs, united
        logs.setdefault(app, []).append(Path(log).read_bytes())
    results = {app: verdict(app, log) for app, log in logs.items()}
    for app, (ok, _, why) in results.items():
        print(f"{app}: {'PASS' if ok else 'FAIL'}: {why}")
    found = {app: calls for app, (ok, calls, _) in results.items() if ok}
    if "--write" in argv and found:
        write(found)
    return int(len(found) != len(results))

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
