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
"""The verdicts. A boot passes when its last census is green, every class
the gate requires is up, the rows add up to the summary, and nothing fatal
was said. A run passes when it holds the boots it must, no two the same
log, and every one passes."""

import hashlib

from .census import read

# a reference laptop must bring all of these up on every boot.
LAPTOP = ("battery", "ac", "lid", "keyboard", "pointer", "display", "storage", "network", "usb")


def boot_faults(b, require=()):
    """Why one boot fails; empty when it passes."""
    out = [f"{b.name}: {line}" for line in b.fatal]
    if b.summary is None:
        return out + [f"{b.name}: no [CENSUS] summary line"]
    up, absent, held, missing, verdict = b.summary
    states = [r[0] for r in b.rows.values()]
    counted = (states.count("up"), states.count("absent"), states.count("held"), states.count("missing"))
    if counted != (up, absent, held, missing):
        out.append(f"{b.name}: the census rows do not add up to its summary")
    if verdict != "green" or missing:
        for cls, (state, _, _, why) in sorted(b.rows.items()):
            if state == "missing":
                out.append(f"{b.name}: {cls} missing: {why}")
        out.append(f"{b.name}: census verdict {verdict}, {missing} missing")
    for cls in require:
        state = b.rows.get(cls, ("not in the census",))[0]
        if state != "up":
            out.append(f"{b.name}: {cls} is {state}, must be up")
    return out


def run_faults(logs, boots=1, require=()):
    """`logs`: (name, text) in boot order. Why the run fails; empty on pass."""
    out, seen = [], {}
    if len(logs) < boots:
        out.append(f"{len(logs)} boot logs, the gate needs {boots}")
    for name, text in logs:
        digest = hashlib.blake2b(text.encode("utf-8", "replace"), digest_size=16).hexdigest()
        if digest in seen:
            out.append(f"{name}: the same log as {seen[digest]}; each boot is counted once")
            continue
        seen[digest] = name
        out += boot_faults(read(name, text), require)
    return out


def trace_faults(before, after):
    """The before/after pair: before shows the old order (_REG before ACPI
    mode), after shows ACPI mode first, no EC turned off, the battery
    published. Why the pair does not show the race gone; empty when it does."""
    out = []
    b, a = read("before", before), read("after", after)
    if b.first_reg_line is None or b.mode_line is None or b.mode_line < b.first_reg_line:
        out.append("before: does not show _REG run before ACPI mode (is it the old build?)")
    if a.mode_line is None or a.first_reg_line is None or a.mode_line > a.first_reg_line:
        out.append("after: ACPI mode is not entered before the first _REG")
    if a.ec_off:
        out.append(f"after: the EC was turned off {a.ec_off} time(s)")
    if a.battery_ms is None:
        out.append("after: no [BRINGUP] line with the battery published")
    return out
