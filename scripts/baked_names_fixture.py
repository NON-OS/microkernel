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
"""check_baked_names.py's fixtures: a tree that passes, one missing a
name, one with a wrong namespace."""

import tempfile
from pathlib import Path


def self_test(check) -> int:
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        load = root / "src/kernel_core/process_spawn/capsule_spawn/from_vfs/load"
        load.mkdir(parents=True)
        site = root / "src/x/spawn.rs"
        site.parent.mkdir(parents=True)
        mk = root / "userland/capsule_x/Capsule.mk"
        mk.parent.mkdir(parents=True)
        mk.write_text("CAPSULE_NAMESPACE := systems.nonos.compositor\n"
                      "CAPSULE_SERVICE_ENDPOINT := service:1:compositor\n")
        site.write_text('CapsuleSpecVerified {}\nconst SERVICE_NAME: &str = "compositor";\n')
        (load / "baked_apps.rs").write_text("pub(crate) const APPS: &[(&str, &str)] = &[];\n")
        fams = 'pub(crate) const FAMILIES: &[&str] = &["driver"];\n'
        good = fams + 'CORE: &[(&str, &str)] = &[("compositor", "systems.nonos.compositor")];\n'
        got = []
        for core in (good, fams + "CORE: &[(&str, &str)] = &[];\n",
                     good.replace("systems.nonos.compositor\")", "com.evil\")")):
            (load / "baked_names.rs").write_text(core)
            got.append(check(root))
    if got != [0, 1, 1]:
        print(f"baked-names: self-test failed, got {got} for good, missing, wrong namespace")
        return 1
    print("baked-names: self-test passed: a missing name and a wrong namespace are caught")
    return 0
