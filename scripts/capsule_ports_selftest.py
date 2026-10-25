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

"""The self-test of check_capsule_ports.py: each source of ports, and each way
two of them can name one port, is shown to bite."""

import json
import pathlib
import tempfile


def self_test(clashes):
    with tempfile.TemporaryDirectory() as d:
        base = pathlib.Path(d) / "userland"
        for name, ep in (("capsule_a", "service:4000:a"), ("capsule_b", "service:4000:b")):
            (base / name).mkdir(parents=True)
            (base / name / "Capsule.mk").write_text(f"CAPSULE_SERVICE_ENDPOINT := {ep}\n")
        assert 4000 in clashes(pathlib.Path(d))[0]
        (base / "capsule_b" / "Capsule.mk").write_text("CAPSULE_SERVICE_ENDPOINT := service:4001:b\n")
        assert not clashes(pathlib.Path(d))[0]
        (base / "linux_guests").mkdir()
        (base / "linux_guests" / "Guests.mk").write_text("$(eval $(call LINUX_GUEST,g,4001,4002))\n")
        assert 4001 in clashes(pathlib.Path(d))[0], "a guest on a capsule's port"
        guests = base / "linux_guests"
        (guests / "Guests.mk").write_text("$(eval $(call LINUX_GUEST,g,4003,4004))\n")
        assert not clashes(pathlib.Path(d))[0]
        (guests / "Qwen.mk").write_text("$(eval $(call QWEN_VARIANT,x86_64,q,4004,4005,/bin/q))\n")
        assert 4004 in clashes(pathlib.Path(d))[0], "a guest named through another macro"
        (guests / "Qwen.mk").write_text("LINUX_GUEST_X_SERVICES := 4010 4012\n")
        (guests / "Go.mk").write_text("GO_IDS := $(shell seq 4020 4029)\n")
        assert not clashes(pathlib.Path(d))[0]
        (guests / "Guests.mk").write_text("$(eval $(call LINUX_GUEST,g,4012,4013))\n")
        assert 4012 in clashes(pathlib.Path(d))[0], "a guest on a port a list holds"
        (guests / "Guests.mk").write_text("$(eval $(call LINUX_GUEST,g,4028,4029))\n")
        assert 4028 in clashes(pathlib.Path(d))[0], "a guest in a range another file holds"
        (guests / "Guests.mk").unlink()
        (guests / "Go.mk").write_text("GO_IDS := $(shell seq 4020 4029)\n"
                                      "$(eval $(call LINUX_GUEST,a,4020,4021))\n")
        assert not clashes(pathlib.Path(d))[0], "the holding file's own guest"
        (guests / "Go.mk").write_text("GO_IDS := $(shell seq 4020 4029)\n"
                                      "$(eval $(call LINUX_GUEST,a,4020,4021))\n"
                                      "$(eval $(call LINUX_GUEST,b,4020,4022))\n")
        assert 4020 in clashes(pathlib.Path(d))[0], "two guests of one file on one port"
        (guests / "Go.mk").unlink()
        nix = pathlib.Path(d) / "tools" / "nix"
        nix.mkdir(parents=True)
        entry = {"dir": "userland/capsule_a", "service_endpoint": "service:4000:a"}
        (nix / "capsules.json").write_text(json.dumps([entry]))
        assert not clashes(pathlib.Path(d))[0], "an entry agreeing with its Capsule.mk"
        entry["reply_endpoint"] = "reply:4001:a.reply"
        (nix / "capsules.json").write_text(json.dumps([entry]))
        assert 4001 in clashes(pathlib.Path(d))[0], "a stale entry on another's port"
        (base / "linux_userland").mkdir()
        (base / "linux_userland" / "U.mk").write_text("$(eval $(call LG,py,4030,4031,/p))\n")
        guest = {"dir": "userland/linux_userland", "handle": "linux.userland.py",
                 "service_endpoint": "service:4030:linux.userland.py"}
        (nix / "capsules.json").write_text(json.dumps([guest]))
        assert not clashes(pathlib.Path(d))[0], "a lane guest's own entry"
        guest["handle"] = "linux.userland.gone"
        (nix / "capsules.json").write_text(json.dumps([guest]))
        assert 4030 in clashes(pathlib.Path(d))[0], "a stale entry for a lane guest"
    print("capsule ports: self-test PASS")
