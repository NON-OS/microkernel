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
"""What --strict refuses, and the launcher arm it lets through. Read by
cap_audit_selftest."""

from pathlib import Path
from types import SimpleNamespace

import cap_audit
import cap_audit_kernel as kern
import cap_audit_launch as launch
import cap_audit_userland as user


def check_strict():
    """--strict holds a capsule to its own calls and to an optional Debug;
    EXEMPT and DEBUG_REQUIRED are the only ways out, and a dependency's call
    is not a failure."""
    def report(name, debug=None, needs=None, strength=kern.HARD):
        return {"capsule": name,
                "bits": [{"name": "Debug", "source": debug}] if debug else [],
                "missing": [{"needs": needs, "strength": strength}] if needs else []}
    fails = cap_audit.strict_failures([
        report("app", debug="required"), report("app2", debug="optional"),
        report("tool", needs="Memory"), report("dep", needs="Admin",
                                               strength=kern.CONDITIONAL),
        report("attack", debug="required", needs="Network"),
        report("linux", debug="required")])
    assert fails == ["app: Debug is required; it belongs in CAPSULE_OPTIONAL_CAPS",
                     "tool: would fail without Memory"], fails
    assert "attack" in cap_audit.EXEMPT and "linux" in cap_audit.DEBUG_REQUIRED


def check_launch(root):
    """MkAppLaunch without AppInstall passes for the owner of the shell's
    endpoint or of one SERVES pairs with a listing, and fails for any other
    capsule; the launchers are the kernel's, read from its tree."""
    found = launch.launchers(Path(root).resolve())
    assert {"desktop_shell", "app.simplex"} <= found, found
    gate = kern.Gate("MkAppLaunch", "all", {"AppInstall"}, kern.HARD, "t")
    gate.owners = found
    k = SimpleNamespace(bits={"IPC": 8, "AppInstall": 1 << 30}, syscalls={"MAPL": gate},
                        handlers={}, network=[], network_cite="t", keyed=[],
                        keyed_cite="t", owner_cite="t", acls=[])
    calls = [user.Use("capsule_me", "a:1", {"MAPL"})]

    def strict(own, granted=8):
        me = SimpleNamespace(granted=granted, own_names=[own], service_names=[own],
                             mk=Path("userland/me/Capsule.mk"))
        lacks = cap_audit.assess(me, calls, k, "capsule_me")[1]
        return cap_audit.strict_failures([{"capsule": own, "bits": [], "missing": [
            {"needs": n, "strength": f.strength} for n, f in lacks]}])
    assert strict("app.simplex") == [] and strict("desktop_shell") == []
    for other in ("app.simplex.1", "app.terminal", "simplex"):
        assert strict(other) == [f"{other}: would fail without AppInstall"], other
    assert strict("app.terminal", 8 | 1 << 30) == [], "AppInstall launches as before"
