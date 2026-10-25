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


"""How each boot went, and the verdict over all of them."""

import os
import shlex

from . import watch


# A boot that ran to its end: its patterns seen, or its own exit 0.
RAN = ("seen", "exited 0", "planned")


def start(name, cmd, log, until, settle, bound, cwd=None, env=None):
    try:
        return watch.run(cmd, log, until, settle, bound, cwd, env)
    except OSError as e:
        return f"cannot start {e.filename or cmd[0]}: {e.strerror}"


def prepare(cmd, bound, dry):
    """The copy, its throwaway keys and, for the capsule boot, its image."""
    print(f"selftest: build: {shlex.join(cmd)}", flush=True)
    how = "planned" if dry else start("build", cmd, os.devnull, [], 0, bound)
    return how if how in ("planned", "exited 0") else f"build {how}"


def boot(b, bound, dry):
    print(f"selftest: {b.name}: (in {b.cwd}) {shlex.join(b.cmd)}", flush=True)
    if dry:
        return "planned"
    # A log from an earlier run would satisfy the patterns before this boot
    # printed anything.
    if os.path.exists(b.log):
        os.remove(b.log)
    os.makedirs(os.path.dirname(b.log), exist_ok=True)
    return start(b.name, b.cmd, b.log, b.until, b.settle, bound, b.cwd, b.env)


def verdict(results, check_rc):
    """0 every boot ran and the checker passed; 1 the checker failed or a boot
    did not run to its end; 2 a build failed or a tool could not start."""
    for name, how in results:
        print(f"selftest: {name}: {how}")
    if any(how.startswith(("build ", "cannot start")) for _, how in results) or check_rc == 2:
        return 2
    return 0 if check_rc == 0 and all(h in RAN for _, h in results) else 1
