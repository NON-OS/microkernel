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
"""The decoy: a feature planted and then found, so the check is known to
bite. Read by check_dark_features."""

import tempfile
from pathlib import Path

from dark_features_scan import analyse

def self_test():
    """A decoy feature with cfg sites and no lane must be reported."""
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        (root / "Cargo.toml").write_text(
            '[package]\nname = "x"\n[features]\ndefault = ["lit"]\nlit = []\n'
            'decoy = []\n[dependencies]\n')
        (root / "src").mkdir()
        (root / "src/lib.rs").write_text('#[cfg(feature = "lit")] fn a() {}\n'
                                         '#[cfg(feature = "decoy")] fn b() {}\n'
                                         '#[cfg(feature = "ghost")] fn c() {}\n')
        (root / "mk").mkdir()
        (root / "Makefile").write_text("build:\n\tcargo build --features lit\n")
        (root / ".github/workflows").mkdir(parents=True)
        (root / ".github/workflows/x.yml").write_text(
            "    strategy:\n      matrix:\n        features:\n          - viamatrix\n"
            "    steps:\n      - run: cargo check --features lit,${{ matrix.features }}\n")
        (root / "Cargo.toml").write_text((root / "Cargo.toml").read_text().replace(
            'decoy = []\n', 'decoy = []\nviamatrix = []\n'))
        (root / "src/lib.rs").write_text((root / "src/lib.rs").read_text()
                                         + '#[cfg(feature = "viamatrix")] fn d() {}\n')
        dark, undeclared, _ = analyse(root)
        assert dark == ["decoy"], dark
        assert undeclared == ["ghost"], undeclared
    print("dark-features: self-test passed, the decoy was reported")


