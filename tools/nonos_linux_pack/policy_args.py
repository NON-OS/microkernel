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
"""The policy command's arguments."""

import argparse
from pathlib import Path


def parser(doc: str) -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(prog="nonos-linux-pack policy", description=doc,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--program", type=Path, required=True, help="the program's ELF")
    ap.add_argument("--mask", required=True, help="services, e.g. WAYLAND,MODELS_RO,TTY")
    ap.add_argument("--calls", required=True, help="the set `syscalls --from-log` printed")
    ap.add_argument("--dests", type=Path, help="destinations, one dests= item a line")
    ap.add_argument("--publisher", required=True, help="the publisher key, 64 hex digits")
    ap.add_argument("--app", required=True, help="the listing's app name, e.g. linux.qwen-small")
    ap.add_argument("--release", required=True, help="the release name, e.g. qwen-small@1")
    ap.add_argument("--out", type=Path, help="also write the 256 bytes here")
    return ap
