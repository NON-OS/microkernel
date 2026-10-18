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

"""The packing command's arguments."""

import argparse
from pathlib import Path

from . import __doc__ as DOC


def parser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(description=DOC,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--sysroot", type=Path, required=True,
                    help="root of the Linux distribution the program came from")
    ap.add_argument("--program", type=Path, action="append", required=True,
                    help="program to install, inside the sysroot; repeatable")
    ap.add_argument("--data", action="append", default=[],
                    help="a data file as STOREPATH=DISKPATH, or a path inside "
                         "the sysroot to install where it already sits; "
                         "repeatable. The keymap goes in this way.")
    ap.add_argument("--image", type=Path,
                    help="store image to write into; without it the entries are printed")
    ap.add_argument("--lba", type=int, default=256, help="store LBA in the image")
    ap.add_argument("--pack", type=Path, default=Path("tools/nonos-store-pack"),
                    help="the packer to hand the entries to")
    return ap
