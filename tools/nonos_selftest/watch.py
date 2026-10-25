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


"""One boot, bounded. The command runs in a process group of its own, so the
whole of it (make, nix, QEMU) is stopped at once: when every pattern it waits
for is in its log and `settle` seconds have passed, or at the bound."""

import os
import re
import signal
import subprocess
import time

POLL = 1.0


def text(path):
    try:
        with open(path, errors="replace") as f:
            return f.read()
    except FileNotFoundError:
        return ""


def stop(proc, grace=10):
    """SIGTERM to the group, then SIGKILL to whatever is left of it: the leader
    can exit first and leave a QEMU dying slowly under it."""
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(proc.pid, sig)
        except ProcessLookupError:
            return
        try:
            proc.wait(grace)
        except subprocess.TimeoutExpired:
            pass


def run(cmd, log, until, settle, bound, cwd=None, env=None):
    """How the boot ended: "exited N", "seen", or "bound"."""
    pending = [re.compile(p) for p in until]
    proc = subprocess.Popen(cmd, cwd=cwd, env={**os.environ, **(env or {})}, start_new_session=True)
    deadline, seen_at = time.monotonic() + bound, None
    try:
        while True:
            rc = proc.poll()
            if rc is not None:
                return f"exited {rc}"
            now = time.monotonic()
            if seen_at is None and pending:
                body = text(log)
                pending = [p for p in pending if not p.search(body)]
                if not pending:
                    seen_at = now
            if seen_at is not None and now - seen_at >= settle:
                return "seen"
            if now >= deadline:
                return "bound"
            time.sleep(POLL)
    finally:
        stop(proc)
