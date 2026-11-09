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
"""Which native capsules the deny-kill would end, and on which call.

The kernel ends a native capsule that makes a
syscall its signed manifest does not declare (required and optional bits,
before the boot profile trims them); MkDebug is never a kill, and the Linux
personality keeps EPERM (src/syscall/contract/deny/verdict.rs). This lists,
per capsule, every syscall cap_audit finds reachable from its code or a
crate it links whose capability-table gate the manifest does not satisfy.

`own` is a call in the capsule's own crate; `linked` is a call somewhere in
a crate it links, read whole, so reachable but not shown to be reached.
Both end the capsule if the call is made. A site read by hand and found
not to make the call outside the manifest is listed in REVIEWED with the
reason, for the capsules the reading covers; it is reported as reviewed, not
as a finding. A reading that rests on how the code is shaped carries a guard
(GUARDS) that checks that shape in the tree; when the guard fails, the site
is a finding again. --strict exits 1 on any finding; --self-test checks the
guard and the reading against a tree built to break them.
"""

import argparse
import re
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import cap_audit  # noqa: E402

SYSCALL = re.compile(r"^(\w+) \(([A-Z0-9]{4})\)$")
NEVER_KILLED = {"MDBG"}
PERSONALITY = {"linux"}

# call site (path up to the line) -> (capsules the reading covers, or "*"; why)
HEADLESS = {"desktop-shell", "installer", "market", "net-core", "nsh-attest", "policy", "wallpaper_catalog"}
REVIEWED = {
    "userland/toolkit/src/component_dispatch/render/attached_surface.rs": (
        "*", "attaches only after MkCapCheck says GraphicsSurfaceMap is held "
             "(holds_surface_map); in apps the toolkit's server code is linked, not run"),
    "userland/app_skeleton/src/runner/": (
        HEADLESS, "these capsules use app_skeleton's vfs client and log_line only; "
                  "none runs the window runner"),
    "userland/app_skeleton/src/setup/": (
        HEADLESS, "these capsules use app_skeleton's vfs client and log_line only; "
                  "none opens a window"),
    "userland/app_skeleton/src/swap/": (
        HEADLESS, "the swapchain's surface and fence calls are Swap's, and only "
                  "app_skeleton's setup/ builds or holds a Swap; these capsules open "
                  "no window"),
}

SKELETON = "userland/app_skeleton/src"
SWAP_WORD = re.compile(r"\bSwap\b")


def swap_only_from_setup(root):
    """Whether `Swap` is named in app_skeleton outside swap/ and setup/ (a
    comment aside): a window path elsewhere would reach the fence calls
    without a window opened through setup/."""
    src = root / SKELETON
    for f in sorted(src.rglob("*.rs")):
        rel = f.relative_to(src).as_posix()
        if rel.startswith(("swap/", "setup/")):
            continue
        code = "\n".join(line.split("//", 1)[0] for line in f.read_text().splitlines())
        if SWAP_WORD.search(code):
            return False
    return True


GUARDS = {"userland/app_skeleton/src/swap/": swap_only_from_setup}


def held(root):
    """The reviewed sites whose guard holds in the tree at `root`."""
    return {site for site in REVIEWED if GUARDS.get(site, lambda _: True)(root)}


def reviewed(capsule, at, sites=None):
    for site, (who, why) in REVIEWED.items():
        if sites is not None and site not in sites:
            continue
        if at.startswith(site) and (who == "*" or capsule in who):
            return why
    return None


def findings(reports, sites=None):
    out = {}
    for r in reports:
        if r["capsule"] in PERSONALITY:
            continue
        own = Path(r["dir"]).name
        calls = {}
        for m in r["missing"]:
            for e in m["evidence"]:
                hit = SYSCALL.match(e["what"])
                if not hit or hit.group(2) in NEVER_KILLED:
                    continue
                if reviewed(r["capsule"], e["at"], sites):
                    continue
                where = "own" if e["crate"] == own else "linked"
                key = (hit.group(1), hit.group(2), m["needs"])
                calls.setdefault(key, set()).add((where, e["crate"], e["at"]))
        if calls:
            out[r["capsule"]] = calls
    return out


def self_test():
    """The swap guard bites on a Swap named outside setup/, and a site whose
    guard fails is a finding again."""
    report = [{"capsule": "wallpaper_catalog", "dir": "userland/capsule_wallpaper_catalog",
               "missing": [{"needs": "GraphicsSurfaceCreate", "evidence": [
                   {"what": "MkSurfaceFenceWait (MSFW)", "crate": "app_skeleton",
                    "at": "userland/app_skeleton/src/swap/fence.rs:20"}]}]}]
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        src = root / SKELETON
        for rel, text in {"swap/mod.rs": "pub struct Swap;\n",
                          "setup/open.rs": "use crate::swap::Swap;\n",
                          "clients/vfs.rs": "// a Swap in a comment is not a use\n"}.items():
            (src / rel).parent.mkdir(parents=True, exist_ok=True)
            (src / rel).write_text(text)
        if not swap_only_from_setup(root) or findings(report, held(root)):
            raise SystemExit("cap-audit-kill self-test: the reading was refused on a tree that holds it")
        (src / "runner").mkdir()
        (src / "runner/paint.rs").write_text("fn f(s: &crate::swap::Swap) {}\n")
        if swap_only_from_setup(root) or not findings(report, held(root)):
            raise SystemExit("cap-audit-kill self-test: a Swap outside setup/ did not bring the finding back")
        if findings([dict(report[0], capsule="calculator")], held(root) | set(REVIEWED)) == {}:
            raise SystemExit("cap-audit-kill self-test: a reading covered a capsule it does not name")
    print("cap-audit-kill self-test: ok")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--capsule", action="append", default=[])
    ap.add_argument("--strict", action="store_true")
    ap.add_argument("--self-test", action="store_true")
    args = ap.parse_args()
    if args.self_test:
        self_test()
        return 0
    # Every call site, not cap_audit's first few: a reviewed site must not
    # stand in for one nobody has read.
    cap_audit.KEEP = sys.maxsize
    sites = held(args.root)
    for site in sorted(set(REVIEWED) - sites):
        print(f"cap-audit-kill: the reading of {site} no longer holds; its calls are findings",
              file=sys.stderr)
    found = findings(cap_audit.audit(args.root, tuple(args.capsule)), sites)
    for capsule, calls in sorted(found.items()):
        print(capsule)
        for (variant, tag, needs), sites in sorted(calls.items()):
            kinds = "/".join(sorted({w for w, _, _ in sites}))
            first = sorted(sites, key=lambda s: (s[0] != "own", s[2]))[0]
            print(f"  {variant} ({tag}) needs {needs} [{kinds}] e.g. {first[2]} ({first[1]})")
    print(f"cap-audit-kill: {len(found)} capsule(s) with a reachable call outside the manifest",
          file=sys.stderr)
    return 1 if args.strict and found else 0


if __name__ == "__main__":
    raise SystemExit(main())
