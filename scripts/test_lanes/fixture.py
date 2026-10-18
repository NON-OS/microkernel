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
"""The self-test's tree: a proofs crate that mounts a capsule's tested file, a
marked crate, an unmarked one on the list, and one with test helpers only."""

import subprocess

from .baseline import BASELINE, LANES

PKG = '[package]\nname = "{}"\nversion = "0.1.0"\nedition = "2021"\n'
MARK = "\n[package.metadata.nonos]\ntest-lane = true\n"
TEST = "#[cfg(test)]\nmod tests {\n    #[test]\n    fn holds() {}\n}\n"
LISTED = "# the format is in the header\nunmarked a lead passes on the host\n"
BL, LJ = str(BASELINE), str(LANES)

BASE = {
    "userland/x_proofs/Cargo.toml": PKG.format("x_proofs"),
    "userland/x_proofs/Cargo.lock": "",
    "userland/x_proofs/src/lib.rs": '#[path = "../../capsule/src/ring.rs"]\nmod ring;\nmod local;\n',
    "userland/x_proofs/src/local.rs": TEST,
    "userland/capsule/Cargo.toml": PKG.format("capsule"),
    "userland/capsule/src/main.rs": "#![no_std]\n#![no_main]\nmod ring;\n",
    "userland/capsule/src/ring.rs": TEST,
    "marked/Cargo.toml": PKG.format("marked") + MARK,
    "marked/Cargo.lock": "",
    "marked/src/lib.rs": TEST,
    "unmarked/Cargo.toml": PKG.format("unmarked"),
    "unmarked/src/lib.rs": "mod inner;\n",
    "unmarked/src/inner.rs": TEST,
    "helpers/Cargo.toml": PKG.format("helpers"),
    "helpers/src/lib.rs": "#[cfg(test)]\npub fn helper() {}\n",
    LJ: '{\n "marked": ""\n}\n',
    BL: LISTED,
}


def skipping(args):
    """The marked crate running only some of its test targets."""
    return {"marked/Cargo.toml": PKG.format("marked") + MARK + f'test-args = "{args}"\n',
            "marked/tests/kept.rs": TEST, "marked/tests/skipped.rs": TEST,
            LJ: '{\n "marked": "%s"\n}\n' % args}


def workspace(args):
    """A marked workspace root and a member with no lock of its own."""
    return {"ws/Cargo.toml": PKG.format("ws") + MARK + f'test-args = "{args}"\n'
            + '\n[workspace]\nmembers = ["core"]\n', "ws/Cargo.lock": "", "ws/src/lib.rs": "",
            "ws/core/Cargo.toml": PKG.format("core"), "ws/core/src/lib.rs": TEST,
            LJ: '{\n "marked": "",\n "ws": "%s"\n}\n' % args}


def build(root, edits):
    """Write the base tree with the edits over it (None deletes) and make it a repository."""
    for rel, text in {**BASE, **edits}.items():
        if text is not None:
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text(text)
    subprocess.run(["git", "init", "-q", str(root)], check=True)
