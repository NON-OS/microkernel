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
"""MkAppLaunch's launcher arm, read from the kernel. Read by cap_audit.

The cap table admits MkAppLaunch for AppInstall, or for a caller owning an
endpoint the kernel registered at spawn: the desktop shell's, or a capsule's
SERVES pairs with a listing (`caller_launches`, capability.rs). The table
parser reads only bits, so a launcher without AppInstall read as one whose
launch fails. The handler holds a SERVES capsule to its own listing
(app_launch_rule.rs); owning the endpoint is what the table asks. The arm is
read from the text it rests on, and the audit stops when that text moves.
"""

import re
from pathlib import Path

import cap_audit_kernel as kern

CAPABILITY = Path("src/syscall/contract/capability.rs")
SERVES = Path("src/services/registry/serves.rs")
TAG = "MAPL"
GATE = "fn caller_launches() -> bool {"
TERMS = ('caller_owns("desktop_shell")',
         "LISTINGS.iter().any(|&(capsule, _)| caller_owns(capsule))")


def launchers(root):
    """The endpoint names whose owner MkAppLaunch admits without AppInstall."""
    text = (root / CAPABILITY).read_text()
    at = text.find(GATE)
    body = text[at:text.find("\n}\n", at)] if at >= 0 else ""
    if not all(term in body for term in TERMS):
        raise kern.SourceMoved(f"{CAPABILITY} no longer admits launchers as {TERMS}")
    table = (root / SERVES).read_text()
    start = table.find("pub(crate) const LISTINGS")
    rows = table[start:table.find("];", start)] if start >= 0 else ""
    served = re.findall(r'^\s*\("([a-z0-9._-]+)", "[^"]+"\),', rows, re.M)
    if not served:
        raise kern.SourceMoved(f"{SERVES} lists no capsule's listing in LISTINGS")
    return frozenset(re.findall(r'caller_owns\("([^"]+)"\)', body) + served)


def syscall_gates(root):
    """cap_audit_kernel's gates, the launch's naming the endpoints whose
    owner it admits whatever bits it holds."""
    gates = kern.syscall_gates(root)
    if TAG not in gates:
        raise kern.SourceMoved(f"the kernel enum no longer declares {TAG}")
    gates[TAG].owners = launchers(root)
    return gates


def asked(tags, gates, capsule):
    """The syscall tags whose gates `capsule` must meet with its bits: every
    one, but a launch by the owner of an endpoint the launch names."""
    own = set(capsule.service_names)
    return {t for t in tags if not getattr(gates.get(t), "owners", frozenset()) & own}
