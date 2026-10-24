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


"""The logs read, and the reports written. A file the check cannot use is
exit 2, never a verdict."""

import hashlib
import sys


def fail(message):
    print(f"attack suite: {message}", file=sys.stderr)
    raise SystemExit(2)


def read(paths):
    """Each log as (path, text, sha256 of the bytes that text was read from)."""
    out = []
    for p in paths:
        try:
            with open(p, "rb") as f:
                body = f.read()
        except OSError as e:
            fail(f"cannot read {p}: {e.strerror}")
        out.append((p, body.decode("utf-8", errors="replace"), hashlib.sha256(body).hexdigest()))
    return out


def write(path, body):
    try:
        with open(path, "w") as f:
            f.write(body)
    except OSError as e:
        fail(f"cannot write {path}: {e.strerror}")
