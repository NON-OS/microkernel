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
"""What check_release_string reads: the files that show the release, and the
string literals in every app's own source."""

import re

# The files that put the release on a screen: setup, Settings, About and the
# Terminal's version line, fetch card and banner.
SURFACES = [
    "userland/capsule_install/brand/src/release.rs",
    "userland/capsule_settings/src/settings/ui/build_info.rs",
    "userland/capsule_about/src/about/data/build.rs",
    "userland/capsule_terminal/src/command/builtin/version.rs",
    "userland/capsule_terminal/src/command/builtin/neofetch/gather.rs",
    "userland/capsule_terminal/src/paint/fetch_version.rs",
]
EMBED = re.compile(r'include_(str|bytes)!\("(\.\./)+VERSION"\)')
LITERAL = re.compile(r'"(?:[^"\\\n]|\\.)*"')
# The product's name, as written in source, then a release number.
NAMED = re.compile(r'N(?:Ø|O|\\u\{d8\})NOS\W+v?\d+\.\d+')
SKIP = ("/tests/", "/upstream-src/", "/target/")


def sources(root):
    """Path to text for every app source file outside tests and vendored trees."""
    out = {}
    for p in sorted((root / "userland").glob("*/src/**/*.rs")):
        rel = p.relative_to(root).as_posix()
        if "_proofs/" in rel or any(s in "/" + rel for s in SKIP):
            continue
        out[rel] = p.read_text(errors="replace")
    for s in SURFACES:
        if s not in out and (root / s).exists():
            out[s] = (root / s).read_text()
    return out


def code_lines(text):
    """Line number and line, with comment lines left out."""
    for n, line in enumerate(text.splitlines(), 1):
        if not line.lstrip().startswith(("//", "*", "/*")):
            yield n, line


def problems(root, files):
    version = (root / "VERSION").read_text().strip()
    found = [f"{s} no longer embeds VERSION" for s in SURFACES
             if not EMBED.search(files.get(s, ""))]
    for path, text in sorted(files.items()):
        for n, line in code_lines(text):
            for lit in LITERAL.findall(line):
                if version in lit or NAMED.search(lit):
                    found.append(f"{path}:{n} spells a release: {lit}")
    return found
