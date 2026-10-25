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
"""check_devnet_feature.py --self-test: each rule passes the tree's own shapes
and refuses each way a release could be given the local test chain."""

import json

import devnet_feature_scan as scan
import devnet_feature_tree as tree

TOML = ('[package]\nname = "c"\n\n[features]\n# turns devnet on for capsules-dev\n'
        'devnet = [\n  "nonos_nox_config/devnet",\n  "nonos_route_link/devnet",\n]\nwide = ["x"]\n\n'
        '[dependencies]\nnonos_nox_config = { path = "../nonos_nox_config" }\n')
MK = "# the dev set knows the local chain\nCAPSULE_DEV_CARGO_FEATURES := devnet\ninclude nonos-mk/capsule.mk\n"
ENTRY = {"slug": "nox-settle", "cargo_features": "", "dev_only": False}
DEV = {"slug": "nox-settle", "dev_cargo_features": "devnet"}  # its capsules-dev.json row
CAT, DEVCAT = "tools/nix/capsules.json", "tools/nix/capsules-dev.json"
NAMES = '{ }:\nlet\n  names = [\n    "rpc.devnet.nonos.test"\n    "10.0.2.2"\n  ];\nin names\n'
PROFILE = 'standard = { kernel = [ "microkernel-full-gui" ]; loader = "standard"; };'


def cat(path, row, **kw):
    return scan.catalogue(path, json.dumps([dict(row, **kw)]), tree.CATALOGUES[path])


def self_test():
    cases = [
        (scan.cargo("c/Cargo.toml", TOML), True),
        (scan.cargo("c/Cargo.toml", TOML.replace('"../nonos_nox_config" }', '"../n", features = ["devnet"] }')), False),
        (scan.cargo("c/Cargo.toml", TOML + '\n[dependencies.nonos_route_link]\nfeatures = ["devnet"]\n'), False),
        (scan.cargo("c/Cargo.toml", TOML.replace('wide = ["x"]', 'wide = ["x", "devnet"]')), False),
        (scan.cargo("c/Cargo.toml", TOML.replace('wide = ["x"]', 'default = ["nonos_nox_config/devnet"]')), False),
        (scan.capsule_mk("u/Capsule.mk", MK), True),
        (scan.capsule_mk("u/Capsule.mk", MK + "CAPSULE_CARGO_FEATURES := devnet\n"), False),
        (cat(CAT, ENTRY), True),
        (cat(DEVCAT, DEV), True),
        (cat(CAT, ENTRY, cargo_features="devnet"), False),
        (cat(CAT, ENTRY, cargo_features="a,devnet"), False),
        (cat(CAT, ENTRY, dev_cargo_features="devnet"), False),
        (cat(DEVCAT, DEV, cargo_features="devnet"), False),
        (scan.anywhere(scan.NAMES, NAMES), True),
        (scan.anywhere(scan.NAMES, NAMES + "# cargo build --features devnet\n"), True),
        (scan.anywhere(scan.NAMES, NAMES.replace("in names", "in names ++ [ \"--features devnet\" ]")), False),
        (scan.anywhere("tools/nix/checks.nix", NAMES), False),
        (scan.anywhere("tools/nix/config.nix", PROFILE), True),
        (scan.anywhere("tools/nix/config.nix", PROFILE.replace('gui" ]', 'gui" "devnet" ]')), False),
        (scan.anywhere("nonos.toml", 'profile = "standard"\nfeatures = ["devnet"]\n'), False),
        (scan.anywhere(".github/workflows/r.yml", "run: cargo build --release --features devnet\n"), False),
    ]
    wrong = [i for i, (failed, ok) in enumerate(cases) if (not failed) != ok]
    if wrong:
        print(f"devnet-feature: self-test failed on cases {wrong}")
        return 1
    print(f"devnet-feature: self-test passed, {sum(not ok for _, ok in cases)} bad inputs rejected "
          f"and {sum(ok for _, ok in cases)} good accepted")
    return 0
