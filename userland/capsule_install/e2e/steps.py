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
"""The steps a run takes against the machine: wait, shot, key, type, until."""

import os
import time


def until(serial, text, secs):
    end = time.time() + secs
    while time.time() < end:
        with open(serial, errors="replace") as f:
            if text in f.read():
                return True
        time.sleep(1)
    return False


def step(q, s, out, serial):
    kind, _, arg = s.partition(":")
    if kind == "wait":
        time.sleep(float(arg))
    elif kind == "shot":
        q.shot(os.path.join(out, arg + ".png"))
    elif kind == "key":
        q.key(*arg.split("+"))
        time.sleep(0.3)
    elif kind == "type":
        q.type_text(arg)
    elif kind == "until":
        text, _, secs = arg.rpartition(":") if arg.count(":") else (arg, "", "600")
        ok = until(serial, text, float(secs or 600))
        print(f"until {text!r}: {'seen' if ok else 'NOT SEEN'}", flush=True)
        return ok
    return True
