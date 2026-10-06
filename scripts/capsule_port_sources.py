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
"""Where capsule ports are declared: each userland/*/Capsule.mk, and the Linux
guest makefiles. A guest is enrolled like a capsule, so its ports share the one
space; the guest makefiles are only read when NONOS_LINUX_GUESTS=1, which is
why a check of Capsule.mk alone missed them. The production Linux userland
(userland/linux_userland) declares its programs the same way.

A guest makefile gives a guest its pair as two adjacent literal arguments of a
call, LINUX_GUEST or a macro over it such as QWEN_VARIANT, the guest's name
the argument before them. A file that makes guests in a loop holds their ports
in a variable named *_SERVICES, *_REPLIES or *_IDS, set to a list of ports or
to $(shell seq FIRST LAST); every port in it is that file's, owned as
"<file>:*", for its own guests and no one else."""

import re

ENDPOINT = re.compile(r"\b(service|reply):(\d+):([\w.]+)")
KEYS = ("CAPSULE_SERVICE_ENDPOINT", "CAPSULE_REPLY_ENDPOINT", "CAPSULE_INSTANCE_ENDPOINTS")
CALL = re.compile(r"\$\(call \w+,([^()]*)")
HELD = r"^(\w+_(?:SERVICES|REPLIES|IDS))\s*:?=\s*"
LIST = re.compile(HELD + r"(\d+(?:\s+\d+)*)\s*$")
SEQ = re.compile(HELD + r"\$\(shell seq (\d+) (\d+)\)\s*$")


def capsule_ports(mk):
    joined = re.sub(r"\\\n", " ", mk.read_text())
    for line in joined.splitlines():
        if line.strip().startswith(KEYS):
            for kind, port, name in ENDPOINT.findall(line):
                yield int(port), f"{mk.parent.name} {kind} {name}"


def guest_pair(args):
    for i in range(1, len(args) - 1):
        if args[i].isdigit() and args[i + 1].isdigit():
            return args[i - 1], int(args[i]), int(args[i + 1])
    return None


def guest_ports(root):
    # A guest's owner is file and name, so one guest declared in two files is
    # two owners. The test guests and the production userland share a space.
    lanes = ("userland/linux_guests/*.mk", "userland/linux_userland/*.mk")
    for mk in sorted(m for lane in lanes for m in root.glob(lane)):
        for line in re.sub(r"\\\n", " ", mk.read_text()).splitlines():
            if line.lstrip().startswith("#"):
                continue
            for args in CALL.findall(line):
                pair = guest_pair(args.split(","))
                if pair:
                    name, service, reply = pair
                    yield service, f"{mk.name}:{name} service linux.guest.{name}"
                    yield reply, f"{mk.name}:{name} reply linux.guest.{name}"
            if m := LIST.match(line):
                for port in m.group(2).split():
                    yield int(port), f"{mk.name}:* holds {m.group(1)}"
            if m := SEQ.match(line):
                for port in range(int(m.group(2)), int(m.group(3)) + 1):
                    yield port, f"{mk.name}:* holds {m.group(1)}"
