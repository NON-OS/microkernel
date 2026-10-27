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

"""One step of a drive: the forms nonos-drive's own help lists."""

import sys
import time

from steps import until

from .dump import dump
from .plug import unplug


def rel(q, dx, dy):
    q.cmd("input-send-event", events=[
        {"type": "rel", "data": {"axis": "x", "value": dx}},
        {"type": "rel", "data": {"axis": "y", "value": dy}},
    ])


def absolute(q, x, y, w=1920, h=1080):
    """The pointer to pixel (x, y) of a w x h screen, for a machine with an
    absolute device (nonos-qemu --lane usb-tablet): QEMU's axes run 0 to 0x7fff."""
    scale = lambda v, n: max(0, min(0x7FFF, v * 0x7FFF // max(1, n - 1)))
    q.cmd("input-send-event", events=[
        {"type": "abs", "data": {"axis": "x", "value": scale(x, w)}},
        {"type": "abs", "data": {"axis": "y", "value": scale(y, h)}},
    ])


def click(q, button="left"):
    for down in (True, False):
        event = {"type": "btn", "data": {"down": down, "button": button}}
        q.cmd("input-send-event", events=[event])
        time.sleep(0.08)


def run(q, s, out, serial):
    kind, _, arg = s.partition(":")
    if kind == "until":
        timed = arg.rsplit(":", 1)[-1].isdigit()
        text, _, secs = arg.rpartition(":") if timed else (arg, "", "600")
        ok = until(serial, text, float(secs or 600))
        print(f"  until {text!r}: {'seen' if ok else 'NOT SEEN'}", flush=True)
        if not ok:
            dump(q, out)
        return ok
    if kind == "wait":
        time.sleep(float(arg))
    elif kind == "shot":
        q.shot(str(out / f"{arg}.png"))
        print(f"  shot {arg}.png", flush=True)
    elif kind == "key":
        q.key(*arg.split("+"))
        time.sleep(0.3)
    elif kind == "type":
        q.type_text(arg)
    elif kind == "move":
        dx, dy = (int(v) for v in arg.split(","))
        rel(q, dx, dy)
        time.sleep(0.1)
    elif kind == "abs":
        absolute(q, *(int(v) for v in arg.split(",")))
        time.sleep(0.1)
    elif kind == "click":
        click(q, arg or "left")
    elif kind == "unplug":
        return unplug(q, arg)
    else:
        sys.exit(f"nonos-drive: unknown step {s!r}")
    return True
