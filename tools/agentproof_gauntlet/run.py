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
"""tools/agentproof-gauntlet --job DIR: one agent, built into a gauntlet image,
booted headless on the IOMMU lane, judged from its serial log.

Exit 0 when the box refused nothing (or no more than --max-refused), the
program started and the report came whole; 1 when it did not; 2 when the
image could not be built or booted. result.json and serial.log go to --out.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path

from .identity import commit, job_hash
from .verdict import judge, read

ROOT = Path(__file__).resolve().parents[2]
IMAGE = ROOT / "target/dev/tree/target/release/gauntlet-dev/nonos.img"
DONE = r"\[AGENTPROOF\] v=1 status=(done|error|refused)"


def boot(log: Path, timeout: int) -> int:
    cmd = ["nix", "run", ".#qemu", "--", "--image", str(IMAGE), "--tpm", "--headless", "--fresh",
           "--serial", str(log), "--lane", "iommu=intel", "--smp", "4", "--mem", "4G",
           "--timeout", str(timeout), "--expect", DONE]
    return subprocess.run(cmd, cwd=ROOT).returncode


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="agentproof-gauntlet", description=__doc__.split("\n")[0])
    ap.add_argument("--job", type=Path, required=True, help="a folder: `job` and the agent's files")
    ap.add_argument("--out", type=Path, default=ROOT / "target/agentproof")
    ap.add_argument("--max-refused", type=int, default=0)
    ap.add_argument("--timeout", type=int, default=1800, help="seconds the boot may take")
    ap.add_argument("--no-build", action="store_true", help="boot the gauntlet image already built")
    a = ap.parse_args(argv)
    a.out.mkdir(parents=True, exist_ok=True)
    # The tree's commit as the image is copied from it, not as the run ends.
    source = commit(ROOT)
    if not a.no_build:
        build = ["nix", "develop", "--command", "python3", "tools/nonos-dev-image",
                 "--profile", "gauntlet", "--agentproof-job", str(a.job.resolve())]
        if subprocess.run(build, cwd=ROOT).returncode != 0:
            print("agentproof-gauntlet: the image did not build", file=sys.stderr)
            return 2
    log = a.out / "serial.log"
    log.write_text("")
    booted = boot(log, a.timeout)
    result = read(log.read_text(errors="replace"))
    why = judge(result, a.max_refused)
    result.update(passed=not why, why=why, booted=booted, keys="test keys (development copy)",
                  job_sha256=job_hash(a.job), image_commit=commit(IMAGE.parents[3]),
                  source_commit=source)
    (a.out / "result.json").write_text(json.dumps(result, indent=1) + "\n")
    for line in result["out"]:
        print(f"  | {line}")
    print("agentproof-gauntlet: " + ("PASS" if not why else "FAIL: " + "; ".join(why)))
    if result["status"] is None and booted != 0:
        return 2
    return 0 if not why else 1
