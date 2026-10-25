#!/usr/bin/env bash
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

# SPHINCS+, NTRU and Classic McEliece do not ship (src/crypto/pqc/unvetted):
# the `pqc-unvetted` feature that builds them must never be turned on by a
# default, a profile, a build target or CI. The only places the name may
# appear are its own definition and comments in Cargo.toml, this check, the
# kernel sources that test for it, documents, and the one type-check line in
# mk/55-dark-features.mk, the compile-only `make check-dark-features` that
# scripts/check_proof_features.py holds to compiling and nothing else.

set -euo pipefail

bad=0

# In Cargo.toml: no feature list may name it, the default list included.
if grep -nE '"pqc-unvetted"' Cargo.toml; then
    echo "Cargo.toml: a feature list turns on pqc-unvetted" >&2
    bad=1
fi

# Anything that builds: make, CI, scripts, nix, workflows.
places=(Makefile mk nonos-ci tools scripts .github flake.nix nix)
existing=()
for p in "${places[@]}"; do
    [ -e "${p}" ] && existing+=("${p}")
done
if hits="$(grep -rnI 'pqc-unvetted' "${existing[@]}" 2>/dev/null \
    | grep -v '^nonos-ci/check-pqc-unvetted.sh:' \
    | grep -v '^nonos-ci/run-static-checks.sh:' \
    | grep -vxE 'mk/55-dark-features.mk:[0-9]+:	\$\(DARK_CHECK\) --features pqc-unvetted')" \
    && [ -n "${hits}" ]; then
    printf '%s\n' "${hits}" >&2
    echo "a build or CI file names pqc-unvetted" >&2
    bad=1
fi

exit "${bad}"
