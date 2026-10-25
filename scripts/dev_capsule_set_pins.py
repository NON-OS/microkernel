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
"""What scripts/check_dev_capsule_set.py holds the tree to: each line a release
depends on, each way a release could reach the development set, and the
mutants its self-test makes of them."""

FARM = "(lib.mapAttrsToList (n: v: { name = n; path = v; }) capsules."
# What must still hold, file by file.
PINS = {
    "tools/nix/image.nix": ["capsuleSet = cfg: if cfg.release then capsules.bySlug else capsules.bySlugDev;",
                            "cp ${(capsuleSet cfg).${e.slug}}/${e.bin} ${e.dir}/", "${placeCapsules cfg}"],
    "tools/nix/artifacts.nix": ["cp ${(image.capsuleSet cfg).${e.slug}}/${e.bin} $out/capsules/"],
    "tools/nix/default.nix": [f'capsules = pkgs.linkFarm "nonos-capsules" {FARM}bySlug);',
                              f'capsules-dev = pkgs.linkFarm "nonos-capsules-dev" {FARM}bySlugDev);'],
    # The seal refuses a release the set by these names (release_set.py).
    "tools/nix/capsules.nix": ['name = "nonos-capsule-${e.slug}${e.suffix or ""}";', 'suffix = "-dev";'],
    "tools/nonos_seal/capsules.py": ['return "capsules-dev" if DEVELOPMENT else "capsules"'],
    "tools/nonos_seal/__main__.py": ['capsules.DEVELOPMENT = not cfg["release"]',
                                     'release_set.refuse(capsules_out, cfg["release"])',
                                     'verify.reproduced(step, attr, elf, efi, capsules_out, cfg["release"])'],
    "tools/nonos_seal/verify.py": ["rebuilt = nix_build(step, capsules.attr())\n    release_set.refuse(rebuilt, release)"],
}
# The set named past capsuleSet, or the seal building one by name, not capsules.attr().
ABSENT = {
    "tools/nix/image.nix": ["capsules.bySlug.", "capsules.bySlugDev."],
    "tools/nix/artifacts.nix": ["bySlug"],
    "tools/nonos_seal/__main__.py": ['nix_build(step, "capsules'],
    "tools/nonos_seal/verify.py": ['nix_build(step, "capsules'],
}
MUTANTS = [
    ("tools/nix/image.nix", "then capsules.bySlug else capsules.bySlugDev", "then capsules.bySlugDev else capsules.bySlug"),
    ("tools/nix/artifacts.nix", "(image.capsuleSet cfg).${e.slug}", "capsules.bySlugDev.${e.slug}"),
    ("tools/nix/artifacts.nix", "(image.capsuleSet cfg)", "(image.capsuleSet (cfg // { release = false; }))"),
    ("tools/nix/default.nix", f'"nonos-capsules" {FARM}bySlug)', f'"nonos-capsules" {FARM}bySlugDev)'),
    ("tools/nix/capsules.nix", 'suffix = "-dev";', 'suffix = "";'),
    ("tools/nonos_seal/capsules.py", "if DEVELOPMENT", "if not DEVELOPMENT"),
    ("tools/nonos_seal/__main__.py", 'DEVELOPMENT = not cfg["release"]', 'DEVELOPMENT = cfg["release"]'),
    ("tools/nonos_seal/__main__.py", '    release_set.refuse(capsules_out, cfg["release"])\n', ""),
    ("tools/nonos_seal/__main__.py", "nix_build(step, capsules.attr()), False", 'nix_build(step, "capsules-dev"), False'),
    ("tools/nonos_seal/verify.py", "release_set.refuse(rebuilt, release)", "release_set.refuse(rebuilt, False)"),
]
