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
"""A crates.io tool held to the tool sandbox. Read by cap_audit."""


def sandbox_report(capsule, kernel, grant):
    """For a crates.io tool: the grant it is spawned with, the sandbox bits
    its manifest lacks (the spawn gate refuses the grant), and the bits vfs
    asks that it is not installed with (it cannot open a file). None for a
    capsule that is not a crates.io tool."""
    registry = capsule.bin_name in kernel.sandboxed
    if not registry and capsule.fields.get("CAPSULE_DOMAIN") != "crates.io":
        return None
    if registry:
        grant = kernel.sandbox
    installed = capsule.required | (capsule.optional & (grant or 0))
    return {"grant": f"0x{(grant or 0):X}",
            "from": kernel.sandbox_cite if registry else (capsule.mirror or "its manifest"),
            "lacks": kernel.names(kernel.sandbox & ~capsule.granted),
            "vfs_unheld": kernel.names(kernel.vfs_bits & ~installed)}
