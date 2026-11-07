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

"""Alpine's index as the device reads it (capsule_linux install/index.rs,
pkg.rs): records end at a blank line, `D:` drops conflicts, paths, `cmd:`
and `pc:` and every version constraint, `p:` adds names and sonames. Each
lookup answers the first match in index order, as the device's linear
finds do."""

import re
from typing import NamedTuple

CUT = re.compile(r"[<>=~]")


class Pkg(NamedTuple):
    name: str
    version: str
    checksum: str
    depends: tuple
    install_if: tuple
    branch: int
    size: int
    installed: int


def depends(line: str) -> tuple:
    keep = [t for t in line.split() if not t.startswith(("!", "/", "cmd:", "pc:"))]
    return tuple(CUT.split(t)[0] for t in keep)


def number(text: str) -> int:
    return int(text) if text.isascii() and text.isdigit() else 0


class Index:
    def __init__(self):
        self.pkgs, self.libs, self.names, self.own = [], {}, {}, {}
        self.providers = {"lib": {}, "name": {}}

    def add(self, raw: bytes, branch: int) -> None:
        cur, provides = {}, []
        for line in [x[:-1] if x.endswith("\r") else x for x in raw.decode("utf-8", "replace").split("\n")] + [""]:
            if not line:
                self.finish(cur, provides, branch)
                cur, provides = {}, []
                continue
            key, rest = line[0], line[2:]
            if key == "p":
                provides = [t.split("=")[0] for t in rest.split()]
            elif key in "CPSIVDi":
                cur[key] = rest

    def finish(self, cur: dict, provides: list, branch: int) -> None:
        if not cur.get("P") or not cur.get("V"):
            return
        at = len(self.pkgs)
        p = Pkg(cur["P"], cur["V"], cur.get("C", ""), depends(cur.get("D", "")),
                tuple(CUT.split(t)[0] for t in cur.get("i", "").split()), branch,
                number(cur.get("S", "")), number(cur.get("I", "")))
        self.pkgs.append(p)
        self.own.setdefault(p.name, at)
        self.provide("name", p.name, at)
        for name in provides:
            if name.startswith("so:"):
                self.provide("lib", name[3:], at)
            elif ":" not in name:
                self.provide("name", name, at)

    def provide(self, kind: str, key: str, at: int) -> None:
        (self.libs if kind == "lib" else self.names).setdefault(key, at)
        self.providers[kind].setdefault(key, []).append(at)

    def by_lib(self, so: str):
        return self.pkgs[self.libs[so]] if so in self.libs else None

    def by_name(self, name: str):
        return self.pkgs[self.names[name]] if name in self.names else None

    def listed(self, name: str):
        return self.pkgs[self.own[name]] if name in self.own else None

    def resolve(self, want: str, listed: bool):
        if want.startswith("so:"):
            return self.by_lib(want[3:])
        return self.listed(want) if listed else self.by_name(want)

    def provided_in(self, want: str, done: set) -> bool:
        kind, key = ("lib", want[3:]) if want.startswith("so:") else ("name", want)
        return any(self.pkgs[i].name in done for i in self.providers[kind].get(key, ()))
