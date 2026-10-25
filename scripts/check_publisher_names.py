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
"""A capsule a publisher signs declares only names under its namespace: the
kernel refuses any other at load (`from_vfs/load/name_rule.rs`), so it would
install and never start. Held here over tools/nix/capsules.json; exit 1 when
one breaks it. `--self-test` checks the rule against cases."""

import json, os, sys

TOP = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))


def stem(endpoint):
    """`service:<port>:<name>` or `reply:<port>:endpoint.<x>.reply` to its name."""
    kind, _, name = endpoint.split(":", 2)
    if kind == "reply" and name.startswith("endpoint.") and name.endswith(".reply"):
        return name[len("endpoint."):-len(".reply")]
    return name

def under(name, owner):
    return name == owner or name.startswith(owner + ".")

def bad(row):
    """The names of `row` the kernel would refuse at load."""
    ns = row.get("namespace", "")
    if ns == "systems.nonos" or ns.startswith("systems.nonos."):
        return []
    ends = [row.get("service_endpoint", ""), row.get("reply_endpoint", "")]
    ends += row.get("instance_endpoints", "").split()
    return [e for e in ends if e and not under(stem(e), ns)]


def self_test():
    ok = {"namespace": "com.example.x", "service_endpoint": "service:1:com.example.x",
          "reply_endpoint": "reply:2:endpoint.com.example.x.reply"}
    cases = [
        (ok, 0),
        (dict(ok, service_endpoint="service:1:app.x"), 1),
        (dict(ok, reply_endpoint="reply:2:endpoint.app.x.reply"), 1),
        (dict(ok, service_endpoint="service:1:com.example.xy"), 1),
        (dict(ok, instance_endpoints="service:3:com.example.x.1 service:4:app.x.2"), 1),
        (dict(ok, namespace="systems.nonos.x", service_endpoint="service:1:app.x"), 0),
    ]
    for row, want in cases:
        assert len(bad(row)) == want, (row, bad(row))
    return print("publisher names: self-test PASS") or 0


def main(argv):
    if "--self-test" in argv:
        return self_test()
    rows = json.load(open(os.path.join(TOP, "tools/nix/capsules.json")))
    found = [(r.get("slug"), e) for r in rows for e in bad(r)]
    for slug, e in found:
        print(f"publisher names: {slug} declares {e}, outside its namespace", file=sys.stderr)
    print(f"publisher names: {len(rows)} capsules, {len(found)} names outside their namespace")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
