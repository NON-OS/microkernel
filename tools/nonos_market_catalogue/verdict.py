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
"""The boot session's verdict on the catalogue (HANDOFF/catalogue/05):

    PYTHONPATH=tools python3 -m nonos_market_catalogue.verdict [--write] APP=LOG ...

An app passes when its logs (one per debug run; several unite) show the
install start in its release's box, no policy refusal, and a measured set.
--write puts each passing set into its recipe. Exit 0 when every app passes."""

import json
import sys
from pathlib import Path

from nonos_linux_pack.syscalls import buckets, market_set

RECIPES = Path("userland/capsule_market/linux-catalogue.json")
STARTED, REFUSED = b"[LINUX-INSTALL] started, in its release's box linux.", (
    b"[LINUX] refused the market's policy", b"the market does not vouch for its box")


def verdict(app: str, log: bytes):
    """(ok, calls hex or "", why) for one app's logs."""
    if STARTED + app.encode() not in log:
        return False, "", "the install never started in its release's box"
    if any(r in log for r in REFUSED):
        return False, "", "the install role refused the policy"
    try:
        sets = buckets(log)
    except ValueError as why:
        return False, "", f"its measured set is malformed: {why}"
    if not sets:
        return False, "", "no measured set: run it on a debug build and prove it"
    return True, market_set(sets), "installed in its box, set measured"


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
        logs[app] = logs.get(app, b"") + Path(log).read_bytes() + b"\n"
    results = {app: verdict(app, log) for app, log in logs.items()}
    for app, (ok, _, why) in results.items():
        print(f"{app}: {'PASS' if ok else 'FAIL'}: {why}")
    found = {app: calls for app, (ok, calls, _) in results.items() if ok}
    if "--write" in argv and found:
        write(found)
    return int(len(found) != len(results))

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
