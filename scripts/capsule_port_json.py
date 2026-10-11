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

"""tools/nix/capsules.json, the list nix builds capsules from, names each
capsule's endpoints a second time. It is generated from the Capsule.mk files,
but a stale entry still enrols its ports, so they share the one space. An entry
is owned by its directory's name, as its Capsule.mk's ports are, so the two
agreeing is one owner and the two differing is a clash. An entry for a guest of
a Linux lane (a directory of guest makefiles, no Capsule.mk) is owned as
guest_ports owns that guest, "<file>:<name>", its name the handle's last part."""

import json
import pathlib

from capsule_port_sources import ENDPOINT, guest_ports

FIELDS = ("service_endpoint", "reply_endpoint", "instance_endpoints")


def json_ports(root):
    path = root / "tools/nix/capsules.json"
    if not path.exists():
        return
    guests = {who.split()[0] for _, who in guest_ports(root)}
    for entry in json.loads(path.read_text()):
        who = owner(root, entry, guests)
        for field in FIELDS:
            for kind, port, name in ENDPOINT.findall(entry.get(field, "")):
                yield int(port), f"{who} {kind} {name}"


def owner(root, entry, guests):
    where = root / entry["dir"]
    if (where / "Capsule.mk").exists():
        return where.name
    name = entry.get("handle", "").rsplit(".", 1)[-1]
    files = sorted(m.name for m in where.glob("*.mk") if f"{m.name}:{name}" in guests)
    return f"{files[0]}:{name}" if len(files) == 1 else where.name
