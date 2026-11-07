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

"""Content kept between runs, keyed by its pin: <cache>/<B>.tar. Read back
only when it still holds the pin; written only once it does. The cache is
$NONOS_LINUX_SOURCES_CACHE, else ~/.cache/nonos/linux-sources, outside any
tree, so the dev image's copy and the checkout share it."""

import os
import shutil
from pathlib import Path

from .pins import holds


def where() -> Path:
    named = os.environ.get("NONOS_LINUX_SOURCES_CACHE", "")
    return Path(named) if named else Path.home() / ".cache/nonos/linux-sources"


def take(cache: Path, package: str, pin: tuple, out: Path) -> bool:
    """Copy the cached content of `pin` to `out` when it holds; a cached file
    that does not is dropped."""
    kept = cache / f"{pin[0]}.tar"
    if not kept.is_file():
        return False
    if not holds(kept, package, pin):
        kept.unlink()
        return False
    part = out.with_name(out.name + ".part")
    shutil.copyfile(kept, part)
    part.replace(out)
    return True


def keep(cache: Path, tar: Path, pin: tuple) -> None:
    """Keep `tar`, already held to `pin`."""
    cache.mkdir(parents=True, exist_ok=True)
    part = cache / f"{pin[0]}.tar.part"
    shutil.copyfile(tar, part)
    part.replace(cache / f"{pin[0]}.tar")
