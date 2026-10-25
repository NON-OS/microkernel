#!/usr/bin/env python3
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
"""Every capsule a release profile ships has its sealed trust set.

The kernel embeds each shipped capsule's certificate, manifest and STARK
trailer from nonos-data/trust/capsules, which only the release seal writes.
Given the slugs the release profiles ship (tools/nix/image.nix, shipped), this
fails on any without all three files, but for the slugs the baseline names,
which wait on the next seal. The baseline may only shrink: a slug sealed since
and still listed fails too, so the list is always exactly what is missing.
"""

import argparse
import json
import sys
from pathlib import Path

from release_trust_lists import drift, read_baseline, read_doc, unnamed

CAPSULES = Path("tools/nix/capsules.json")


def unsealed(entries, slugs, exists):
    """The shipped slugs missing a cert, manifest or trailer, sorted."""
    wanted = set(slugs)
    return sorted(e["slug"] for e in entries if e["slug"] in wanted
                  and not all(exists(e[k]) for k in ("cert", "manifest", "trailer")))


def verdict(missing, baseline, shipped=None):
    """What is wrong, one line each; empty when the tree matches the baseline."""
    new = [f"release trust: {s} ships in a release profile with no sealed trust set; "
           "seal it, or it stays out of the release profiles" for s in missing if s not in baseline]
    gone = lambda s: shipped is not None and s not in shipped
    done = [f"release trust: {s} is " + ("no longer shipped by any release profile" if gone(s) else "sealed now")
            + "; take it out of the baseline" for s in sorted(baseline) if s not in missing]
    return new + done


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("slugs", nargs="*", help="the capsules the release profiles ship")
    ap.add_argument("--baseline", help="the slugs waiting on the next seal")
    ap.add_argument("--doc", help="the document whose unsealed list must equal the baseline")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args(argv)
    if a.self_test:
        from release_trust_selftest import self_test
        return self_test(unsealed, verdict)
    entries = json.loads(CAPSULES.read_text())
    missing = unsealed(entries, a.slugs, lambda p: Path(p).is_file())
    baseline = read_baseline(a.baseline) if a.baseline else set()
    found = (unnamed(entries, a.slugs) + verdict(missing, baseline, set(a.slugs))
             + (drift(baseline, read_doc(a.doc)) if a.doc else []))
    for line in found:
        print(line, file=sys.stderr)
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
