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

"""The literals in source that name a place off the machine: a URL or a host
name in a string, an IPv4 address written as four bytes, and a build-time
setting that names a mirror, host or endpoint."""

import re

from .lexer import code, host_of, uncomment

TLDS = ("net|org|com|io|co|systems|xyz|dev|app|eth|onion|anyone|ai|cloud|info|me|tv|cc"
        "|de|uk|fr|nl|ch|eu|us|ca|jp|ru|cn|se|no|fi|at|be|pl|cz|sh|to|gg|ly|is|im")
HOST = re.compile(r'["\']((?:https?|wss?)://[^"\'\s]+|[a-z0-9-]+(?:\.[a-z0-9-]+)*\.(?:%s))'
                  r'[:/"\']' % TLDS, re.I)
QUAD = re.compile(r"(?:\[|Ipv4Addr::new\()\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,"
                  r"\s*(\d{1,3})\s*[\])]")
PORT_AFTER = re.compile(r"\s*,\s*\d{2,5}\s*\)")
# Words that mark a file as one about addresses; four bytes elsewhere are
# tables, colours and test data.
ADDRESSY = re.compile(r"u8; ?4|ipv4|\bip\b|addr|gateway|mix|authorit|relay|server|host"
                      r"|peer|resolver|dns|ntp")
SETTING = re.compile(r'option_env!\(\s*"([A-Z0-9_]*'
                     r'(?:MIRROR|HOST|URL|RPC|PEER|ENDPOINT)[A-Z0-9_]*)"')
TEST_FILE = re.compile(r"(^tests?\.rs$|_tests?\.rs$|_fuzz\.rs$|^test_.*\.py$|fixture|_test_)")
# A host built at run time from a template is no fixed place.
NAMED = re.compile(r"^[A-Za-z0-9.\[\]:-]+$")


def in_file(name, text):
    """Each (literal, line number) in one source file."""
    quotes = "\"'" if name.endswith(".py") else '"'
    lines = [code(line, quotes) for line in uncomment(text).splitlines()]
    addressy = bool(ADDRESSY.search((name + " " + " ".join(lines)).lower()))
    for n, line in enumerate(lines, 1):
        for host in (host_of(m.group(1)).lower() for m in HOST.finditer(line)):
            if NAMED.match(host):
                yield host, n
        for m in SETTING.finditer(line):
            yield "build:" + m.group(1), n
        for m in QUAD.finditer(line):
            quad = m.groups()
            port = PORT_AFTER.match(line[m.end():])
            if all(int(b) < 256 for b in quad) and (port or addressy):
                yield ".".join(quad), n


def found(dirs, root):
    """Every literal under the given crate or tool directories, with where."""
    out = {}
    for d in dirs:
        for f in sorted(d.rglob("*")):
            rel = f.relative_to(root).as_posix()
            if f.suffix not in (".rs", ".py") or "/target/" in rel or "upstream" in rel:
                continue
            if "/tests/" in rel or TEST_FILE.search(f.name):
                continue
            for lit, n in in_file(f.name, f.read_text(errors="replace")):
                out.setdefault(lit, []).append(f"{rel}:{n}")
    return out
