#!/usr/bin/env python3
# Every image starts every CPU the machine has (cpu-2). 0.9.2 shipped kernels
# that ran on the boot CPU alone: the nix profiles carried nonos-smp, but the
# nonos-mk lanes built without it and booted QEMU with -smp 1, and nothing
# failed. This fails when any way of building or booting drops back to one CPU:
#   - nonos.toml does not say smp = true, or config.nix's default does not, or
#     a profile's or a feature group's list names nonos-smp to take it out;
#   - the mk lanes' one kernel build does not add NONOS_SMP_FEATURE, or that is
#     not nonos-smp unless NONOS_SMP=0 is asked for;
#   - make itself, asked what nonos-mk-core builds (no build runs), leaves the
#     feature out;
#   - any mk QEMU lane says -smp 1.
# `check-smp-default.py [root]`, the checkout by default.

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
FEATURE = "NONOS_SMP_FEATURE := $(if $(filter 0,$(NONOS_SMP)),,$(_boot_comma)nonos-smp)"


def config(root: Path) -> list[str]:
    errs = []
    if not re.search(r"(?m)^smp = true\s*$", (root / "nonos.toml").read_text()):
        errs.append("nonos.toml does not say smp = true")
    nix = (root / "tools/nix/config.nix").read_text()
    if not re.search(r"(?m)^\s*smp = true;", nix):
        errs.append("tools/nix/config.nix's default is not smp = true")
    for name, body in re.findall(r"(?ms)^\s*(drop|\w+Features)\s*=\s*([^;]*);", nix):
        if "nonos-smp" in body:
            errs.append(f"tools/nix/config.nix: {name} takes nonos-smp out")
    return errs


def lanes(root: Path) -> list[str]:
    errs = []
    mk = "\n".join(p.read_text() for p in sorted((root / "mk").glob("*.mk")))
    build = re.search(r"(?s)define nonos_kernel_build\n(.*?)endef", mk)
    if not build or "--features $(2)$(NONOS_SMP_FEATURE)" not in build.group(1):
        errs.append("mk: nonos_kernel_build does not add $(NONOS_SMP_FEATURE)")
    if not re.search(r"(?m)^NONOS_SMP \?= 1$", mk) or FEATURE not in mk:
        errs.append("mk: NONOS_SMP is not on by default, or does not mean nonos-smp")
    for line in re.findall(r"(?m)^.*-smp 1\b.*$", mk):
        errs.append(f"mk: a QEMU lane boots one CPU: {line.strip()[:80]}")
    return errs


def asked(root: Path) -> list[str]:
    cmd = ["make", "--no-print-directory", "-C", str(root), "-n", "-W", "Cargo.toml"]
    env = dict(os.environ, NONOS_IN_FLAKE="1")
    run = subprocess.run([*cmd, "nonos-mk-core"], env=env, capture_output=True, text=True)
    lines = re.findall(r"--no-default-features --features (\S+)", run.stdout)
    if run.returncode != 0 or not lines:
        return ["make -n nonos-mk-core printed no kernel build"]
    if any("nonos-smp" not in features.split(",") for features in lines):
        return [f"make builds a kernel without nonos-smp: {lines[0]}"]
    return []


def main() -> int:
    errs = config(ROOT) + lanes(ROOT) + asked(ROOT)
    for err in errs:
        print(f"check-smp-default: {err}")
    return 1 if errs else 0


if __name__ == "__main__":
    sys.exit(main())
