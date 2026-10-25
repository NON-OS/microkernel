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

"""A small repository for the audit's tests: one networked capsule that
builds from a library, one that holds no Network, and the list naming what
the networked one reaches."""

from pathlib import Path

MK = "CAPSULE_REQUIRED_CAPS    := {caps}\n"
LIB = 'pub const PEER: ([u8; 4], u16) = ([203, 0, 113, 7], 443);\n'
NET = (
    'const URL: &str = "https://dir.example.net/v1"; // see https://docs.example.org\n'
    'const HOST: &str = option_env!("NONOS_X_MIRROR").unwrap_or("");\n'
)
QUIET = 'const PIXELS: [u8; 4] = [10, 20, 30, 40];\nconst S: &str = "https://quiet.example.com";\n'
PAGE = """# Privacy

| Capsule | Why |
|---|---|
| `capsule_dial` | It dials |

| Address | Who |
|---|---|
| `203.0.113.7`, `dir.example.net` | dial |

| Build setting | Who |
|---|---|
| `NONOS_X_MIRROR` | dial |
"""


def put(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def make(root: Path):
    u = root / "userland"
    put(u / "capsule_dial/Capsule.mk", MK.format(caps="0x1C"))
    put(u / "capsule_dial/Cargo.toml", '[dependencies]\nnetlib = { path = "../netlib" }\n')
    put(u / "capsule_dial/src/main.rs", NET)
    put(u / "capsule_dial/src/tests.rs", 'const T: &str = "https://test.example.io";\n')
    put(u / "netlib/Cargo.toml", "[package]\n")
    put(u / "netlib/src/lib.rs", LIB)
    put(u / "capsule_quiet/Capsule.mk", MK.format(caps="0x18"))
    put(u / "capsule_quiet/src/main.rs", QUIET)
    put(root / "docs/security/privacy.md", PAGE)
    return root
