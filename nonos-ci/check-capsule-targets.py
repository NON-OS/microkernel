#!/usr/bin/env python3
# Every capsule the make rule builds is built for the user target the build
# asks for. Until C-23 the rule's cargo line read `--target
# ../x86_64-nonos-user.json` whatever NONOS_USER_TARGET said, so the aarch64
# lanes embedded x86_64 capsules, or none, and nothing failed.
#
# For each user target, make itself is asked (no build runs):
#   - each capsule's `<slug>_TARGET`, the target its ELF path is under;
#   - `make -n -B` of every `nonos-mk-<slug>`, whose cargo line names the
#     target it would really build for.
# A capsule built for a target that is not a NONOS user target names its own
# (CAPSULE_TARGET: the Linux guests and userland, built for musl) and must
# keep it. Every other capsule must follow the user target, in its variable
# and in its cargo line. "Its own" is never inferred from the target staying
# put across runs: that is exactly what the defect looked like.

from __future__ import annotations

import os
import re
import subprocess
import sys

TARGETS = ("x86_64-nonos-user", "aarch64-nonos-user")
MARK = "NONOS-CI-CAPSULE "
CARGO = re.compile(r"cd (\S+) && .*\bcargo (?:build|rustc) .*--target \.\./(\S+)\.json")


def make(target: str, *args: str) -> str:
    env = dict(os.environ, NONOS_IN_FLAKE="1")
    cmd = ["make", "--no-print-directory", f"NONOS_USER_TARGET={target}", *args]
    run = subprocess.run(cmd, env=env, capture_output=True, text=True, check=False)
    if run.returncode != 0:
        sys.stderr.write(run.stdout[-2000:] + run.stderr[-2000:])
        raise SystemExit(f"check-capsule-targets: make failed for {target}")
    return run.stdout


def capsules(target: str) -> dict[str, tuple[str, str, bool]]:
    """slug -> (dir, <slug>_TARGET, prebuilt), as make sees them."""
    each = "$(foreach s,$(NONOS_VERIFIED_CAPSULES) $(NONOS_DEV_CAPSULES)," \
        f"$(info {MARK}$(s) $($(s)_DIR) $($(s)_TARGET) $(if $($(s)_PREBUILT_BIN),1,0)))"
    out = make(target, f"--eval=nonos-ci-capsules: ; @:{each}", "nonos-ci-capsules")
    found = {}
    for line in out.splitlines():
        if line.startswith(MARK):
            slug, path, built_for, prebuilt = line[len(MARK):].split()
            found[slug] = (path, built_for, prebuilt == "1")
    return found


def cargo_targets(target: str, slugs: list[str]) -> dict[str, set[str]]:
    out = make(target, "-n", "-B", *(f"nonos-mk-{s}" for s in slugs))
    seen: dict[str, set[str]] = {}
    for path, built_for in CARGO.findall(out):
        seen.setdefault(path, set()).add(built_for)
    return seen


def main() -> int:
    views = {t: capsules(t) for t in TARGETS}
    slugs = sorted(views[TARGETS[0]])
    if not slugs or set(slugs) != set(views[TARGETS[1]]):
        print("check-capsule-targets: make names no capsules, or different ones per target")
        return 1
    own = {s for s in slugs if all(views[t][s][1] not in TARGETS for t in TARGETS)}
    errors = []
    for target in TARGETS:
        built = [s for s in slugs if not views[target][s][2]]
        lines = cargo_targets(target, built)
        for slug in built:
            path, built_for, _ = views[target][slug]
            want = built_for if slug in own else target
            if built_for != want:
                errors.append(f"{slug}: {target} build puts the ELF under {built_for}")
            if lines.get(path) != {want}:
                errors.append(f"{slug}: {target} build runs cargo for {lines.get(path)}, not {want}")
    for error in errors:
        print(f"::error::{error}")
    if errors:
        return 1
    print(f"check-capsule-targets: {len(slugs)} capsules follow the user target "
          f"({len(own)} name their own: {', '.join(sorted(own)) or 'none'})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
