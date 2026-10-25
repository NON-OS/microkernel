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
"""Keep the boot-proof features out of every profile and image.

nonos-shootdown-trace, nonos-smp-park-proof, nonos-smp-stuck-proof,
nonos-smp-pge-proof, nonos-panic-proof and nonos-fault-proof change how a
kernel boots on purpose: one prints every early shootdown, one holds CPU 1
back so the boot CPU parks it, one stalls CPU 1 so the next shootdown fails
closed, one sets CR4.PGE on the boot CPU alone before the kernel does, one
panics init and one raises a kernel #UD once the spawn plan has run. The
kernel refuses them beside nonos-production and nonos-release at compile
time (src/smp/ap/park_proof.rs, src/memory/paging/manager/shootdown/trace.rs,
src/memory/mmu/mmu/protect/cr4/pge.rs, src/userspace/init/injected.rs). That
catches a release build,
not a development or shared image that carries one in, so:

  1. In Cargo.toml each is defined as `name = []` and named in no other
     feature's list.
  2. No profile, image list, make lane, CI script or workflow names one:
     nonos.toml, tools/nix, mk, nonos-ci and .github are searched whole.
     The one exception is mk/55-dark-features.mk, the compile-only lane
     `make check-dark-features`, and only while it stays compile-only: one
     target, every recipe line `$(DARK_CHECK) --features ...`, DARK_CHECK a
     debug `cargo check` with NONOS_SIGNING_KEY removed from the environment,
     and nothing in the file that runs nix or QEMU, builds, signs or packs.

    scripts/check_proof_features.py [--self-test]
"""

import re
import sys
from pathlib import Path

PROOFS = (
    "nonos-shootdown-trace",
    "nonos-smp-park-proof",
    "nonos-smp-stuck-proof",
    "nonos-smp-pge-proof",
    "nonos-panic-proof",
    "nonos-fault-proof",
)
SEARCHED = ("nonos.toml", "tools/nix", "mk", "nonos-ci", ".github")
SUFFIXES = {".toml", ".nix", ".mk", ".sh", ".yml", ".yaml", ".json", ".py"}


def cargo_features(text):
    """Each `name = [ ... ]` of [features], as name -> the text of its list."""
    out = {}
    m = re.search(r"^\[features\]\s*$(.*?)(?=^\[|\Z)", text, re.M | re.S)
    if not m:
        return out
    body = re.sub(r"#[^\n]*", "", m.group(1))
    for f in re.finditer(r"^\s*([A-Za-z0-9_-]+)\s*=\s*\[(.*?)\]", body, re.M | re.S):
        out[f.group(1)] = f.group(2)
    return out


def check_cargo(text):
    feats = cargo_features(text)
    failed = []
    for p in PROOFS:
        if p not in feats:
            failed.append(f"Cargo.toml does not define {p}")
        elif feats[p].strip():
            failed.append(f"{p} turns on other features: [{feats[p].strip()}]")
        for name, members in feats.items():
            if name != p and re.search(rf'"{re.escape(p)}"', members):
                failed.append(f"feature {name} enables {p}")
    return failed


DARK_LANE = "mk/55-dark-features.mk"
# The include guard and the target: the only lines besides DARK_CHECK and recipes.
LANE_FRAME = (
    "ifndef NONOS_DARK_FEATURES_MK",
    "NONOS_DARK_FEATURES_MK := 1",
    ".PHONY: check-dark-features",
    "check-dark-features:",
    "endif",
)
DARK_RECIPE = re.compile(r"^\t\$\(DARK_CHECK\) --features [A-Za-z0-9_,-]+$")
DARK_FORBIDDEN = re.compile(r"--release|\bnix\b|qemu|nonos_kernel_build|\bbuild\b|sign|esp|image|run")


def check_dark_lane(text):
    """Why mk/55-dark-features.mk is no longer a compile-only lane, if it is not."""
    failed = []
    body = [line for line in text.split("\n") if line.strip() and not line.startswith("#")]
    define = " ".join(line.strip() for line in body if not line.startswith("\t$(DARK_CHECK)")
                      and line not in LANE_FRAME)
    if not define.startswith("DARK_CHECK = env -u NONOS_SIGNING_KEY ") or " cargo check " not in define:
        failed.append("DARK_CHECK is not a cargo check with the signing key unset")
    if DARK_FORBIDDEN.search(define.replace("build-std", "")):
        failed.append("DARK_CHECK does more than type-check")
    for line in body:
        if line.startswith("\t") and not DARK_RECIPE.match(line):
            failed.append(f"recipe line is not a DARK_CHECK: {line.strip()}")
        if line.endswith(":") and line != "check-dark-features:":
            failed.append(f"a second target: {line}")
    return failed


def check_tree(files):
    failed = []
    for path, text in files:
        named = [p for p in PROOFS if p in text]
        if named and Path(path).as_posix() == DARK_LANE:
            failed += [f"{path}: {why}" for why in check_dark_lane(text)]
            continue
        failed += [f"{path} names {p}" for p in named]
    return failed


def tree_files(root):
    for top in SEARCHED:
        base = root / top
        paths = [base] if base.is_file() else sorted(base.rglob("*")) if base.is_dir() else []
        for path in paths:
            if path.is_file() and (path.suffix in SUFFIXES or path.name == "Makefile"):
                yield path, path.read_text(errors="replace")


LANE = (
    "# compile only\n.PHONY: check-dark-features\n"
    "DARK_CHECK = env -u NONOS_SIGNING_KEY cargo check --locked "
    "--target x86_64-nonos.json -Zbuild-std=core,alloc\n"
    "check-dark-features:\n\t$(DARK_CHECK) --features nonos-smp,nonos-smp-park-proof\n"
)


def self_test():
    good = '[features]\nnonos-shootdown-trace = []\nnonos-smp-park-proof = []\nnonos-smp-stuck-proof = []\nnonos-smp-pge-proof = []\nnonos-panic-proof = []\nnonos-fault-proof = []\nnonos-smp = []\n# "nonos-smp-park-proof" in a comment is fine\nmicrokernel-x = ["nonos-smp"]\n\n[dependencies]\n'
    cases = [
        (check_cargo(good), True),
        (check_cargo(good.replace('microkernel-x = ["nonos-smp"]', 'microkernel-x = ["nonos-smp", "nonos-smp-park-proof"]')), False),
        (check_cargo(good.replace("nonos-smp-stuck-proof = []\n", "")), False),
        (check_cargo(good.replace("nonos-shootdown-trace = []", 'nonos-shootdown-trace = ["dbg-ring"]')), False),
        (check_cargo(good.replace('microkernel-x = ["nonos-smp"]', 'microkernel-x = [\n  "nonos-smp",\n  "nonos-shootdown-trace",\n]')), False),
        (check_tree([("tools/nix/config.nix", 'features = [ "nonos-smp" ];')]), True),
        (check_tree([("tools/nix/config.nix", 'features = [ "nonos-smp" "nonos-smp-stuck-proof" ];')]), False),
        (check_tree([("mk/20-build.mk", "$(_boot_comma)nonos-shootdown-trace")]), False),
        (check_tree([(DARK_LANE, LANE)]), True),
        (check_tree([(DARK_LANE, LANE.replace("check --locked", "build --locked"))]), False),
        (check_tree([(DARK_LANE, LANE.replace(" --locked", " --release --locked"))]), False),
        (check_tree([(DARK_LANE, LANE.replace("env -u NONOS_SIGNING_KEY ", ""))]), False),
        (check_tree([(DARK_LANE, LANE + "\tnix develop --command make nonos-mk-esp\n")]), False),
        (check_tree([(DARK_LANE, LANE + "boot:\n\t$(DARK_CHECK) --features nonos-smp-park-proof\n")]), False),
        (check_tree([("mk/56-other.mk", LANE)]), False),
    ]
    wrong = [i for i, (failed, ok) in enumerate(cases) if (not failed) != ok]
    if wrong:
        print(f"proof-features: self-test failed on cases {wrong}")
        return 1
    print(f"proof-features: self-test passed, {sum(not ok for _, ok in cases)} bad inputs rejected "
          f"and {sum(ok for _, ok in cases)} good accepted")
    return 0


def main():
    if "--self-test" in sys.argv[1:]:
        return self_test()
    root = Path(".")
    failed = check_cargo((root / "Cargo.toml").read_text()) + check_tree(tree_files(root))
    for line in failed:
        print(f"proof-features: {line}")
    if failed:
        return 1
    print("proof-features: the boot proofs are defined, alone, and in no profile, image or lane "
          "but the compile-only check-dark-features")
    return 0


if __name__ == "__main__":
    sys.exit(main())
