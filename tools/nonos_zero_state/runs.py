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

"""The two boots: the first types the secret and powers off under watch, the
second starts from the disk the first left."""

import shutil

from . import leftovers
from .boot import ROOT, Boot
from .memscan import count_and_drop
from .script import drive, stop_for

WIPED = "[SANITIZE] ZeroState shutdown wipe complete"


def first_boot(a, secret, say):
    b = Boot(a.image, a.out / "serial1.log", a.out / "qemu1.log", a.mem)
    try:
        drive(b.q, a.steps, secret, a.out, a.out / "serial1.log") or stop_for("a setup step")
        b.dump(a.out / "before.dump")
        say("in RAM before the power button", count_and_drop(a.out / "before.dump", secret), True)
        drive(b.q, a.off, secret, a.out, a.out / "serial1.log") or stop_for("a power-off step")
        b.held(a.off_secs) or stop_for("the machine to power off")
        say("wipe line on serial", leftovers.said(a.out / "serial1.log", WIPED), True)
        b.dump(a.out / "after.dump")
        say("in RAM after the wipe", count_and_drop(a.out / "after.dump", secret), False)
    finally:
        b.stop()
    shutil.copyfile(ROOT / "target/qemu/stick.img", a.out / "boot1.img")


def second_boot(a, secret):
    b = Boot(a.out / "boot1.img", a.out / "serial2.log", a.out / "qemu2.log", a.mem)
    try:
        drive(b.q, a.after, secret, a.out, a.out / "serial2.log") or stop_for("a second-boot step")
    finally:
        b.stop()
    return leftovers.said(a.out / "serial2.log", secret)
