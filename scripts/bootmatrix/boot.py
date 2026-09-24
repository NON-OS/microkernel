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
"""One boot: start QEMU, watch the serial log, stop it.

The boot is over when every readiness marker has appeared or the budget runs
out. A kill boot ends earlier: as soon as the store reports itself serving,
QEMU dies without warning, and the caller boots the same disk again.
"""

import random
import subprocess
import time

from .verdict import FATAL, READY, STORE_SERVING

POLL = 0.5


def read_log(path):
    try:
        return path.read_text(errors="replace")
    except FileNotFoundError:
        return ""


def fatal_in(text):
    """The first fatal marker the log carries, if it carries one."""
    return next((word for word in FATAL if word in text), None)


def wait_for(proc, log, markers, deadline):
    """The log text once every marker is in it, or why the wait gave up.

    Three ways to give up, and the caller needs to tell them apart: QEMU exited
    on its own, the guest wrote a fatal marker, or the budget ran out. Only the
    last has nothing to say, and reports an empty reason for the caller to name.
    """
    started = time.monotonic()
    while True:
        text = read_log(log)
        if all(m in text for m in markers):
            return text, True, ""
        exit_code = proc.poll()
        if exit_code is not None:
            text = read_log(log)
            if all(m in text for m in markers):
                return text, True, ""
            return text, False, f"qemu exited {exit_code} after {time.monotonic() - started:.1f}s"
        word = fatal_in(text)
        if word:
            return text, False, f"{word} in log after {time.monotonic() - started:.1f}s"
        if time.monotonic() >= deadline:
            return text, False, ""
        time.sleep(POLL)


def boot(argv, log, timeout, kill_at_serving=False):
    """Run QEMU to readiness. Returns (log text, reached, how it ended)."""
    log.unlink(missing_ok=True)
    proc = subprocess.Popen(argv, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    deadline = time.monotonic() + timeout
    try:
        if kill_at_serving:
            text, reached, why = wait_for(proc, log, [STORE_SERVING], deadline)
            if reached:
                # Inside the first seconds of store traffic, off any boundary.
                time.sleep(random.uniform(0.0, 2.0))
                proc.kill()
                return read_log(log), True, "killed while serving"
            return text, False, why or "store never served"
        text, reached, why = wait_for(proc, log, READY, deadline)
        return text, reached, "ready" if reached else why or "timed out"
    finally:
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(10)
            except subprocess.TimeoutExpired:
                proc.kill()
        stderr = proc.stderr.read().decode(errors="replace").strip()
        if stderr:
            log.with_suffix(".qemu-stderr").write_text(stderr + "\n")
