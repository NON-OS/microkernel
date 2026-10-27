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
"""The Debian packages a recipe's needs bring, chosen as the device's deb
install chooses them (install/deb/run.rs and find.rs): each need's first
alternative the index has, by its own name, then by a name it provides;
Depends and Pre-Depends followed to the end. `table` is the verified Kali
index, name -> Packages stanza (nonos-market-catalogue `kali_index`). Its
pins are what asset_needs.py writes into the content."""

from .asset_needs import MAX_NEEDS


def groups(value: str) -> list:
    """`libc6 (>= 2.34), a | b, python3:any` as [[names]], as fields.rs reads it."""
    name = lambda t: t.strip().split(" ")[0].split("(")[0].split(":")[0].split("[")[0]  # noqa: E731
    out = [[name(t) for t in g.split("|") if name(t)] for g in value.split(",")]
    return [g for g in out if g]


def find(table: dict, want: str):
    rec = table.get(want)
    return rec or next((r for r in table.values() if want in sum(groups(r.get("Provides", "")), [])), None)


def closure(table: dict, needs: list) -> list:
    """The stanzas, each once, or ValueError naming a need nothing gives."""
    done, wanted = {}, [[n] for n in reversed(needs)]
    while wanted:
        group = wanted.pop()
        rec = next((r for r in map(lambda n: find(table, n), group) if r), None)
        if rec is None:
            raise ValueError(f"nothing in the Debian index provides {' | '.join(group)}")
        if rec["Package"] in done:
            continue
        if len(done) == MAX_NEEDS:
            raise ValueError(f"the closure passes {MAX_NEEDS} packages")
        done[rec["Package"]] = rec
        for field in ("Pre-Depends", "Depends"):
            wanted.extend(reversed(groups(rec.get(field, ""))))
    return list(done.values())


def pins(stanzas: list) -> list:
    """asset_needs tuples: name, version, pool file, sha256."""
    return [(r["Package"], r["Version"], r["Filename"], r["SHA256"]) for r in stanzas]
