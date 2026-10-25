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


"""tools/nonos-selftest [options]: boot the three self-test images, then judge
every attack each printed.

  tools/nonos-selftest                 build the selftest image, boot all three
  tools/nonos-selftest --only capsule  one boot (capsule, attest, linux)
  tools/nonos-selftest --iommu         the capsule boot behind a vIOMMU
  tools/nonos-selftest --dry-run       print the commands, run nothing

The logs and the report are under --out (target/selftest): capsule.log,
attest.log, linux.log, report.txt and report.json. Exit 0 when every boot ran
and every attack was refused; 1 when one escaped, never ran, or a boot did
not finish; 2 when a build failed or a tool could not start."""

import argparse
import os
import sys

from . import boots, build, check, guard, run


def main(argv=None):
    ap = argparse.ArgumentParser(prog="nonos-selftest", description=__doc__.split("\n\n")[0],
                                 epilog=__doc__.split("\n\n", 1)[1],
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--only", action="append", choices=boots.NAMES, help="one boot; repeat for more")
    ap.add_argument("--out", default="target/selftest", help="where the logs and the report go")
    ap.add_argument("--iommu", action="store_true", help="the capsule boot behind a vIOMMU (--lane iommu=intel)")
    ap.add_argument("--no-build", action="store_true", help="use the copy, its keys and its image as they are")
    ap.add_argument("--timeout", type=int, default=3600, help="seconds each boot may take, its build in make's")
    ap.add_argument("--build-timeout", type=int, default=7200, help="seconds the selftest image's build may take")
    ap.add_argument("--dry-run", action="store_true", help="print the commands and run nothing")
    a = ap.parse_args(argv)
    if a.timeout <= 0 or a.build_timeout <= 0:
        ap.error("timeouts are seconds, more than 0")
    if not os.path.isfile(check.CHECK):
        ap.error("run from the repository root")
    root = os.getcwd()
    plan = boots.plan(a.only, root, a.out, a.iommu)
    if why := guard.refusal(plan, root, not a.no_build):
        print(f"selftest: refused: {why}", file=sys.stderr)
        return 2
    results = []
    if not a.no_build:
        results.append(("build", run.prepare(build.prepare([b.name for b in plan]), a.build_timeout, a.dry_run)))
        if results[0][1] not in run.RAN:
            return run.verdict(results, 0)
    results += [(b.name, run.boot(b, a.timeout, a.dry_run)) for b in plan]
    return run.verdict(results, check.judge(plan, os.path.abspath(a.out), a.dry_run))


if __name__ == "__main__":
    sys.exit(main())
